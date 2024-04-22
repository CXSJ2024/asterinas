use alloc::string::String;

use crate::alloc::vec::Vec;
use super::listfattr::list_attr;
use super::Xattr;
use super::XattrEntry;
use crate::Errno;


pub fn get_xattr(abs_path:&str, attr:&str) -> Result<XattrEntry,Errno>{
    let entries: Vec<XattrEntry> = list_attr(abs_path)?;
    for entry in entries{
        if entry.attribute.eq(attr) {
            return Ok(entry);
        }
    }
    Err(Errno::EBADF)
}