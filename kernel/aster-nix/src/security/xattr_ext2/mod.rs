use alloc::vec::Vec;

use crate::fs::ext2::{Ext2, Inode};

pub mod getfattr;
pub mod setfattr;
pub mod listfattr;

pub fn test(){

}


pub struct Xattr{
    fs: Arc<Ext2>,
    xattr_block: Arc<Inode>,
    xattr_mapping: Vec<XattrMap> 
}

struct XattrMap{
    attribute: [u8;32],
    ino:u32
}

impl Xattr{
    pub fn xattr_ext2_init(fs: Arc<Ext2>) -> Self{

    }
}

