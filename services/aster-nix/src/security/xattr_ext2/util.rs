use alloc::{format, string::String, vec::Vec};
use super::xattr::XattrEntry;
use crate::Errno;


pub fn encode_xattr_entry(xattr: &XattrEntry) -> Vec<u8> {
    format!("{}|{}|{}\n", xattr.attribute, xattr.value, xattr.file_ino)
        .as_bytes()
        .to_vec()
}

pub fn decode_xattr_entry(data: &mut Vec<u8>) -> Result<XattrEntry, Errno> {
    let str = String::from_utf8_lossy(&data[..]);
    let terms: Vec<&str> = str.split("|").collect();
    if terms.len() == 3 {
        let num = terms[2].parse::<u64>();
        if num.is_err() {
            return Err(Errno::EBADF);
        }
        Ok(XattrEntry {
            attribute: String::from(terms[0]),
            value: String::from(terms[1]),
            file_ino: num.unwrap(),
        })
    } else {
        Err(Errno::EBADF)
    }
}