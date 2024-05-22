// SPDX-License-Identifier: MPL-2.0

use super::SyscallReturn;
use crate::{
    fs::{file_table::FileDesc, inode_handle::InodeHandle},
    prelude::*,
};

pub fn sys_fsync(fd: FileDesc) -> Result<SyscallReturn> {
    debug!("fd = {}", fd);

    let dentry = {
        let current = current!();
        let file_table = current.file_table().lock();
        let file = file_table.get_file(fd)?;
        let inode_handle = file
            .downcast_ref::<InodeHandle>()
            .ok_or(Error::with_message(Errno::EINVAL, "not inode"))?;
        inode_handle.dentry().clone()
    };
    dentry.sync()?;
    Ok(SyscallReturn::Return(0))
}
