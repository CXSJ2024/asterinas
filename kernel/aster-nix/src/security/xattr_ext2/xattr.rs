use alloc::{format, sync::Arc, vec::Vec};

use crate::{
    fs::{
        fs_resolver::{FsPath, FsResolver},
        utils::Inode,
    },
    Errno, String,
};

use spin::MutexGuard;

use crate::security::integrity::{
    ml::{
        entry::MeasurementEntry,
        entry_list::*,
        entry_list
    },
    ima::ima_hash::cal_fd_hash,
};

use super::{set_xattr,XATTR_PATH,IMA_XATTR_KEY};


pub struct Xattr {
    pub xattr_block: Arc<dyn Inode>,
}

#[derive(Debug)]
pub struct XattrEntry {
    pub attribute: String,
    pub value: String,
    pub file_ino: u64,
}

impl Xattr {
    pub fn xattr_handler() -> Result<Self, Errno> {
        let resolver = FsResolver::new();
        if let Ok(path) = FsPath::new(0, XATTR_PATH) {
            let dentry = resolver.lookup(&path).unwrap();
            Ok(Xattr {
                xattr_block: dentry.inode().clone(),
            })
        } else {
            Err(Errno::EBADF)
        }
    }
}



pub fn measure_all(ml:&mut MutexGuard<'static, entry_list::MeasurementList>,resolver: &FsResolver, root_dir: &str) -> crate::prelude::Result<()> {
    let fs_handler = {
        let path = FsPath::new(0, root_dir)?;
        resolver.open(&path, 0, 0)?
    };
    if fs_handler.dentry().type_().is_reguler_file() {
        let algo = select_ima_algo(ml.template);
        let measurement: String = cal_fd_hash(fs_handler.dentry().inode(), 1024, None,Some(root_dir))?.into();
        set_xattr(root_dir, IMA_XATTR_KEY, &measurement)?;
        let template_hash = cal_fd_hash(fs_handler.dentry().inode(), 1024, algo.clone(),Some(root_dir))?.hash.data;
        let content_hash = cal_fd_hash(fs_handler.dentry().inode(), 1024, algo.clone(),None)?.hash.data;
        let entry = MeasurementEntry::new(root_dir,&template_hash,&content_hash);
        ml.add_entry(entry);
    } else if fs_handler.dentry().type_().is_directory() && check_hint(root_dir,ml.appraise){
        let mut items: Vec<String> = Vec::new();
        fs_handler.readdir(&mut items)?;
        for item in items {
            if item == "." || item == ".." {
                continue;
            }
            let abs_path = format!("{}/{}", root_dir, item);
            measure_all(ml,resolver, abs_path.as_str())?;
        }
    }
    Ok(())
}