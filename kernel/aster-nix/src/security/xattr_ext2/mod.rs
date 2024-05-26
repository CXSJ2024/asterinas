use self::setfattr::set_xattr;
use crate::{
    fs::{
        fs_resolver::FsResolver,
        utils::{Inode, InodeMode},
    },
    println,
};

use super::integrity::ml::entry_list;
use xattr::{Xattr,measure_all};

pub mod getfattr;
pub mod listfattr;
pub mod setfattr;
pub mod xattr;
mod util;

const XATTR_PATH: &str = "/xattr";

const USER_EXECUTABLE_PREFIX: &str = "/regression";

const IMA_XATTR_KEY: &str = "security.ima";


pub fn xattr_init() -> Option<Xattr> {
    let fs_resolver = FsResolver::new();
    let root_inode = fs_resolver.root().inode();
    let xattr_inode = root_inode.create(
        &XATTR_PATH[1..],
        crate::fs::utils::InodeType::File,
        InodeMode::all(),
    );
    let mut ml = entry_list::MeasurementList::get_list();
    if let (Ok(inode), Ok(_)) = (
        xattr_inode,
        measure_all(&mut ml,&fs_resolver, USER_EXECUTABLE_PREFIX),
    ) {
        if ml.verify_ml() {
            let _ = Xattr::xattr_handler();
            println!(
                "[kernel] IMA boot measure done, xattr in inode #{}",
                inode.metadata().ino
            );
            Some(Xattr { xattr_block: inode })
        }else{
            println!(
                "[kernel] IMA measurement list verify fail"
            );
            None
        }
    } else {
        println!("[kernel] Securrity Extended File Attribute init fail");
        None
    }
}


