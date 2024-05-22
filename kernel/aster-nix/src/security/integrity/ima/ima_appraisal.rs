use alloc::format;

use super::ima_hash::cal_fd_hash;
use crate::{
    fs::{file_table::FileDesc, inode_handle::InodeHandle, utils::InodeType},
    integrity::ima::ima_hash::{IMAAlogrithm, IMAHash},
    prelude::*,
    security::xattr_ext2::setfattr::set_xattr,
};

pub fn ima_appraisal(fh: &InodeHandle) -> Result<()> {
    let algo: Option<IMAAlogrithm> = None;
    let dentry = fh.dentry();
    let abs_path = &dentry.abs_path();
    if dentry.type_() == InodeType::Dir {
        return Ok(());
    }
    let inode = dentry.inode();
    let res = cal_fd_hash(inode, 1024, algo)?;
    let foo = format! {"{}",res.hash};
    let _ = set_xattr(abs_path, "security.ima", &foo)?;
    //TODO: get xattr from inode
    if res != IMAHash::default() {
        return Err(Error::new(Errno::EIMA));
    }
    Ok(())
}

pub fn ima_appraisal_fd(fd: FileDesc) -> Result<()> {
    let algo: Option<IMAAlogrithm> = None;
    let current = current!();
    let fs = current.fs().read();
    let dentry = fs.lookup_from_fd(fd).unwrap();
    if dentry.type_() == InodeType::Dir {
        return Ok(());
    }
    let inode = dentry.inode();
    let res = cal_fd_hash(inode, 1024, algo)?;
    //TODO: get xattr from inode
    if res != IMAHash::default() {
        return Err(Error::new(Errno::EIMA));
    }
    Ok(())
}

pub fn ima_aduit(fd: FileDesc) -> Result<()> {
    let algo: Option<IMAAlogrithm> = None;
    let current = current!();
    let fs = current.fs().read();
    let dentry = fs.lookup_from_fd(fd).unwrap();
    let inode = dentry.inode();
    let res = cal_fd_hash(inode, 1024, algo)?;
    todo!("store xattr and update the measurement list");
    Ok(())
}
