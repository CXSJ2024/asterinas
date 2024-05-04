use alloc::string::ToString;


use crate::Errno;
use crate::alloc::vec::Vec;
use crate::fs::fs_resolver::FsPath;
use crate::fs::fs_resolver::FsResolver;
use super::Xattr;
use super::XattrEntry;
use super::encode_xattr_entry;

pub fn set_xattr(abs_path: &str,attribute: &str, value: &str) -> Result<(), Errno>{
    let _ = check_perm(attribute)?;
    let handler:Xattr = Xattr::xattr_handler()?;
    if let Ok(path) = FsPath::new(0,abs_path){
        let file = FsResolver::new().lookup(&path);
        if file.is_err() {
            return Err(Errno::ENOENT);
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
        Err(Errno::ENOTXATTR)
    }
}

fn check_perm(attribute: &str) -> Result<(),Errno>{
    let words:Vec<&str> = attribute.split(".").collect();
    if words.len() != 2 {
        return Err(Errno::EPERM)
    }
    let (namespace,field) = (words[0],words[1]);
    match namespace{
        "user" => {
            Ok(())
        },
        "security" => {
            Ok(())
        },
        _ => Err(Errno::EPERM)
    }
}