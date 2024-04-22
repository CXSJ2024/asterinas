use core::borrow::Borrow;
use core::fmt::Display;

use alloc::format;
use alloc::string::ToString;
use alloc::{sync::Arc, vec::Vec};
use crate::fs::fs_resolver::{FsResolver,FsPath};
use crate::security::xattr_ext2::getfattr::get_xattr;
use crate::security::xattr_ext2::listfattr::list_attr;
use crate::{fs::utils::InodeMode, println};
use crate::fs::ext2::Ext2;
use crate::fs::utils::Inode;
use crate::String;
use crate::Errno;

use self::setfattr::set_xattr;


pub mod getfattr;
pub mod setfattr;
pub mod listfattr;

pub struct Xattr{
    pub xattr_block: Arc<dyn Inode>,
}

#[derive(Debug)]
pub struct XattrEntry{
    pub attribute: String,
    pub value: String,
    pub file_ino: u64,
}

const XATTR_PATH :&str = "/xattr";

impl Xattr{
    pub fn xattr_handler() -> Result<Self,Errno>{
        let resolver = FsResolver::new();
        if let Ok(path) = FsPath::new(0, XATTR_PATH){
            let dentry = resolver.lookup(&path).unwrap();
            Ok(Xattr{
                xattr_block: dentry.inode().clone(),
            })
        }else{
            Err(Errno::EBADF)
        }
    }
}

fn encode_xattr_entry(xattr: &XattrEntry) -> Vec<u8>{
    format!("{}|{}|{}\n",xattr.attribute,xattr.value,xattr.file_ino).as_bytes().to_vec()
}

fn decode_xattr_entry(data: &mut Vec<u8>) -> Result<XattrEntry,Errno>{
    let str = String::from_utf8_lossy(&data[..]);
    let terms:Vec<&str> = str.split("|").collect();
    if terms.len() == 3 {
        let num = terms[2].parse::<u64>();
        if num.is_err(){
            return  Err(Errno::EBADF);
        }
        Ok(XattrEntry{
            attribute: terms[0].to_string(),
            value: terms[1].to_string(),
            file_ino: num.unwrap(),
        })
    }else {
        Err(Errno::EBADF)
    }
}



pub fn xattr_init() -> Option<Xattr>{
    let fs_resolver = FsResolver::new();
    let root_inode = fs_resolver.root().inode();
    let xattr_inode = root_inode.create(&XATTR_PATH[1..], crate::fs::utils::InodeType::File, InodeMode::all());
    if let Ok(inode) = xattr_inode {
        println!("[kernel] File extended attribute in inode #{}",inode.metadata().ino);
        Xattr::xattr_handler();
        Some(Xattr{
            xattr_block: inode,
        })
    }else{
        println!("[kernel] Securrity File Extended Attribute fail");
        None
    }
    
}

pub fn test(){
    let abs_path = "/regression/hello_world/hello_world";
    set_xattr(abs_path, "security.ima","hash_data");
    set_xattr(abs_path, "user.field","user value");
    println!("{:?}",list_attr(abs_path).unwrap());
    println!("Get security.ima attribute:{:?}",get_xattr(abs_path, "security.ima").unwrap().value);
}


