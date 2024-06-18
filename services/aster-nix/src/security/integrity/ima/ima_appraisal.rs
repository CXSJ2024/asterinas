const IMA_XATTR: &str = "security.ima";
use super::{
    super::super::xattr_ext2::setfattr::set_xattr,
    ima_hash::{cal_fd_hash, IMAAlogrithm},
};
use crate::{
    fs::{
        file_table::FileDescripter,
        inode_handle::InodeHandle,
        utils::{Dentry, Inode, InodeType},
    },
    prelude::*,
    security::{
        integrity::{
            self,
            ima::ima_hash::IMAHash,
            ml::{
                self,
                entry::MeasurementEntry,
                entry_list::{self, *},
            },
        },
        xattr_ext2::getfattr::get_xattr,
    },
};
//TODO!!!:
//1. hook mmap write back process to do remeasure.
//2. handle output redirection
pub fn ima_appraisal_dentry(dentry: &Dentry) -> Result<()> {
    if integrity::IMA_FEATURE_MODE == 0 {
        return Ok(());
    }
    ima_appraisal_handle(dentry, &dentry.abs_path())
}

pub fn ima_appraisal_fh(fh: &InodeHandle) -> Result<()> {
    if integrity::IMA_FEATURE_MODE == 0 {
        return Ok(());
    }
    let dentry = fh.dentry();
    ima_appraisal_handle(dentry, &dentry.abs_path())
}

pub fn ima_appraisal_fd(fd: FileDescripter) -> Result<()> {
    if integrity::IMA_FEATURE_MODE == 0 || fd == 0 || fd == 1 || fd == 2 {
           return Ok(());
    }
    let current = current!();
    let fs = current.fs().read();
    // let dentry = fs.lookup_from_fd(fd).unwrap();
    match fs.lookup_from_fd(fd)
{    
   Ok(dentry)=> ima_appraisal_handle(&*dentry, &dentry.abs_path()),
   Err(e)=>{
        // println!("fd {}, {:?}",fd,e);
        Ok(())
   }
}
}

pub fn ima_remeasure_fd(fd: FileDescripter) -> Result<()> {
    if integrity::IMA_FEATURE_MODE == 0 || fd == 1 || fd == 2  {
        return Ok(());
    }
    let current = current!();
    let fs = current.fs().read();
    let dentry = fs.lookup_from_fd(fd).unwrap();
    if dentry.inode_type() != InodeType::File {
        return Ok(());
    }
    ima_remeasure_handle(dentry.inode(), &dentry.abs_path())
}

fn ima_appraisal_handle(dentry: &Dentry, abs_path: &str) -> Result<()> {
    if dentry.inode_type() != InodeType::File {
        return Ok(());
    }
    let inode = dentry.inode();
    match get_xattr(abs_path, IMA_XATTR) {
        Ok(val) => {
            let expect = IMAHash::from(val.value);
            let res = cal_fd_hash(inode, 1024, Some(expect.algo.clone()), None)?;
            if res != expect {
                println!(
                    "error!!!\nabs_path: {}\nexpected: {:?}\nactual: {:?}\n",
                    abs_path, expect, res
                );
            }
        }
        Err(_) => {
            //println!("{}'s ima xattr not found, remeasure it", abs_path);
            ima_remeasure_handle(inode, abs_path)?;
        }
    }
    Ok(())
}

fn ima_remeasure_handle(inode: &Arc<dyn Inode>, abs_path: &str) -> Result<()> {
    let mut ml = entry_list::MeasurementList::get_list();
    if !check_hint(abs_path, ml.appraise) {
        return Ok(());
    }
    let hash = if let Ok(val) = get_xattr(abs_path, IMA_XATTR) {
        IMAHash::from(val.value).algo
    } else {
        IMAAlogrithm::default()
    };
    let tpmplate_hash = cal_fd_hash(inode, 1024, Some(hash.clone()), Some(abs_path))?;
    let content_hash = cal_fd_hash(inode, 1024, Some(hash.clone()), None)?;
    ml.add_entry(MeasurementEntry::new(
        abs_path,
        &tpmplate_hash.hash.data,
        &content_hash.hash.data,
    ));
    let _ = ml::sync_write_file(&mut ml);
    let res_string: String = content_hash.into();
    let _ = set_xattr(abs_path, IMA_XATTR, &res_string)?;
    //println!("path: {}\nremeasure result {}", abs_path, &res_string);
    Ok(())
}

fn ima_aduit(fd: FileDescripter) -> Result<()> {
    let algo: Option<IMAAlogrithm> = None;
    let current = current!();
    let fs = current.fs().read();
    let dentry = fs.lookup_from_fd(fd).unwrap();
    let inode = dentry.inode();
    todo!("save to tpm pcr");
    Ok(())
}
