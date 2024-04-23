use crate::println;

use alloc::string::String;
use alloc::string::ToString;

use crate::alloc::vec::Vec;
use super::Xattr;
use super::XattrEntry;
use super::decode_xattr_entry;
use crate::fs::fs_resolver::FsResolver;
use crate::fs::fs_resolver::FsPath;
use crate::Errno;


pub fn list_attr(abs_path:&str) -> Result<Vec<XattrEntry>,Errno>{
    if let Ok(path) = FsPath::new(0,abs_path){
        let file = FsResolver::new().lookup(&path);
        if file.is_err() {
            return Err(Errno::ENOENT);
        }
        let ino = file.unwrap().inode().ino();
        let handler:Xattr = Xattr::xattr_handler()?;

        let mut buf = Vec::new();
        let bytes = handler.xattr_block.read_all(&mut buf).unwrap();
        if bytes <= 0{
            return Err(Errno::ENOTXATTR);
        }
        let content = String::from_utf8_lossy(&buf[..(bytes-1)]);
        let mut lines:Vec<&str> = content.split("\n").collect();
        lines.reverse();

        let mut res = Vec::new();
        for line in lines{
            let entry: XattrEntry = decode_xattr_entry(&mut line.as_bytes().to_vec())?;
            if !contain_xattr(&res,&entry) && ino == entry.file_ino{
                res.push(entry);
            }
        }
        Ok(res)
    }else{
        Err(Errno::ENOTXATTR)
    }
}
    

fn contain_xattr(v: &Vec<XattrEntry>, entry:&XattrEntry) -> bool{
    for e in v{
        if e.file_ino == entry.file_ino && e.attribute.eq(entry.attribute.as_str()) {
            return true;
        }
    }
    return false;
}