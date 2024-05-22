const IMA_XATTR: &str = "security.ima";
use super::{
    super::super::xattr_ext2::setfattr::set_xattr,
    ima_hash::{cal_fd_hash, IMAAlogrithm},
};
use crate::{
    fs::{
        file_table::FileDesc,
        inode_handle::InodeHandle,
        path::Dentry,
        utils::{Inode, InodeType},
    },
    prelude::*,
    security::{integrity::ima::ima_hash::IMAHash, xattr_ext2::getfattr::get_xattr},
};

pub fn ima_appraisal_dentry(dentry: &Dentry) -> Result<()> {
    if dentry.type_() != InodeType::File {
        return Ok(());
    }
    ima_appraisal_handle(dentry.inode(), &dentry.abs_path())
}

pub fn ima_appraisal_ih(fh: &InodeHandle) -> Result<()> {
    let dentry = fh.dentry();
    if dentry.type_() != InodeType::File {
        return Ok(());
    }
    ima_appraisal_handle(dentry.inode(), &dentry.abs_path())
}

pub fn ima_appraisal_fd(fd: FileDesc) -> Result<()> {
    let current = current!();
    let fs = current.fs().read();
    let dentry = fs.lookup_from_fd(fd).unwrap();
    if dentry.type_() != InodeType::File {
        return Ok(());
    }
    ima_appraisal_handle(dentry.inode(), &dentry.abs_path())
}

pub fn ima_remeasure_fd(fd: FileDesc) -> Result<()> {
    let current = current!();
    let fs = current.fs().read();
    let dentry = fs.lookup_from_fd(fd).unwrap();
    if dentry.type_() != InodeType::File {
        return Ok(());
    }
    println!("remeasure file: {}", &dentry.abs_path());
    ima_remeasure_handle(dentry.inode(), &dentry.abs_path())
}

fn ima_appraisal_handle(inode: &Arc<dyn Inode>, abs_path: &str) -> Result<()> {
    match get_xattr(abs_path, IMA_XATTR) {
        Ok(val) => {
            let expect = IMAHash::from(val.value);
            let res = cal_fd_hash(inode, 1024, Some(expect.algo.clone()))?;
            if res != expect {
                println!(
                    "error!!!\nabs_path: {}\nexpected: {:?}\nactual: {:?}\n",
                    abs_path, expect, res
                );
            }
        }
        Err(_) => {
            println!("{}'s ima xattr not found, remeasure it", abs_path);

            let tmp: String = IMAHash::default().into();
            set_xattr(abs_path, IMA_XATTR, &tmp)?;
            ima_remeasure_handle(inode, abs_path)?;
        }
    }
    Ok(())
}

fn ima_remeasure_handle(inode: &Arc<dyn Inode>, abs_path: &str) -> Result<()> {
    let hash = IMAHash::from(get_xattr(abs_path, IMA_XATTR).unwrap().value);
    let res = cal_fd_hash(inode, 1024, Some(hash.algo))?;
    let res_string: String = res.into();
    let _ = set_xattr(abs_path, IMA_XATTR, &res_string)?;
    Ok(())
}

fn ima_aduit(fd: FileDesc) -> Result<()> {
    let algo: Option<IMAAlogrithm> = None;
    let current = current!();
    let fs = current.fs().read();
    let dentry = fs.lookup_from_fd(fd).unwrap();
    let inode = dentry.inode();
    todo!("save to tpm pcr");
    Ok(())
}
