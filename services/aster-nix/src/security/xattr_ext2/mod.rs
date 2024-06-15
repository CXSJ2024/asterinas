use self::setfattr::set_xattr;
use super::integrity::ml::entry_list;
use crate::security::integrity::pcr::{PcrValue, DEFAULT_PCR_REGISTER, PCR_BITSIZE};
use crate::{
    fs::{fs_resolver::FsResolver, utils::InodeMode},
    println,
    security::integrity::ml::{entry_list::PCR, sync_write_file},
};
use xattr::{measure_all, Xattr};

pub mod getfattr;
pub mod listfattr;
pub mod setfattr;
mod util;
pub mod xattr;

pub const XATTR_PATH: &str = "/xattr";

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
    if let (Ok(inode), Ok(_)) = (xattr_inode, {
        let mut res = None;
        for dir in entry_list::FIX_MODE_PREFIX {
            if measure_all(&mut ml, &fs_resolver, dir).is_err() {
            res = Some(dir);
            break;
            }
        }
        if let Some(s) = res {
            Err(s)
        } else {
            Ok(())
        }
    }) {
        if ml.verify_ml() {
            let _ = Xattr::xattr_handler();
            println!("[kernel] IMA boot measure done");
            if PCR::has_pcr() {
                println!(
                    "[kernel] pcr device exist, pcr#{} = {:x?}",
                    DEFAULT_PCR_REGISTER,
                    PCR::op().read_pcr(DEFAULT_PCR_REGISTER)
                );
            }
            let _ = sync_write_file(&mut ml);
            Some(Xattr { xattr_block: inode })
        } else {
            println!("[kernel] IMA measurement list verify fail");
            None
        }
    } else {
        println!("[kernel] Securrity Extended File Attribute init fail");
        None
    }
}
