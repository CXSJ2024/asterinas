use std::println;

use self::xattr_ext2::Xattr;
use super::fs::fs_resolver;

pub mod xattr_ext2;

pub fn security_init(){
    let fs_resolver = FsResolver::new();
    let root_inode = fs_resolver.root().inode().this();
    println!("{}",root_inode.metadata());
    //Xattr::xattr_ext2_init(f)
}