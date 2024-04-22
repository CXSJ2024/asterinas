use alloc::string::ToString;
use core2::error::Error;

use crate::Errno;
use crate::alloc::vec::Vec;
use crate::fs::fs_resolver;
use crate::fs::fs_resolver::FsPath;
use crate::fs::fs_resolver::FsResolver;
use super::Xattr;
use super::XattrEntry;
use super::encode_xattr_entry;

pub fn set_xattr(abs_path: &str,attribute: &str, value: &str) -> Result<(), Errno>{
    let handler:Xattr = Xattr::xattr_handler()?;
    if let Ok(path) = FsPath::new(0,abs_path){
        let file = FsResolver::new().lookup(&path);
        if file.is_err() {
            return Err(Errno::EBADF);
        }
        let data = XattrEntry{
            attribute: attribute.to_string(),
            value: value.to_string(),
            file_ino: file.unwrap().inode().ino(),
        };
        let inode = handler.xattr_block;
        inode.write_at(inode.size(),&encode_xattr_entry(&data)[..]);
        Ok(())
    }else{
        Err(Errno::EBADF)
    }
}