use super::listfattr::list_attr;
use super::xattr::XattrEntry;
use crate::alloc::vec::Vec;
use crate::Errno;

pub fn get_xattr(abs_path: &str, attr: &str) -> Result<XattrEntry, Errno> {
    let entries: Vec<XattrEntry> = list_attr(abs_path)?;
    for entry in entries {
        if entry.attribute.eq(attr) {
            return Ok(entry);
        }
    }
    Err(Errno::ENOTXATTR)
}
