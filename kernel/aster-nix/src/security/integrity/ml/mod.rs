use alloc::string::String;
use aster_frame::ima::tpm::PcrOp;


use crate::{
    fs::{
        fs_resolver::{FsPath, FsResolver},
        utils::InodeMode,
    }, Result
};

use self::entry_list::MeasurementList;

use self::entry::MeasurementEntry;
use spin::MutexGuard;



pub mod entry_list;
pub mod entry;


pub const MEASUREMENT_LIST_ASCII: &str = "/ascii_runtime_measurements";

pub fn measurement_list_init() -> Result<()>{
    MeasurementList::reset_pcr();
    let fs_resolver = FsResolver::new();
    let root_inode = fs_resolver.root().inode();
    let ml_inode = root_inode.create(
        &MEASUREMENT_LIST_ASCII[1..],
        crate::fs::utils::InodeType::File,
        InodeMode::all(),
    )?;
    Ok(())
}

pub fn sync_write_file(ml:&mut MutexGuard<'static, entry_list::MeasurementList>) -> Result<()>{
    let inode = FsResolver::new().lookup(&FsPath::new(0, MEASUREMENT_LIST_ASCII)?)?;
    // let e :String = ml.get_entry(1).unwrap().clone().into();
    // println!("{}",e);
    let entries = ml.get_all();
    let mut idx:usize = 0;
    for e in entries{
        let rec :String = e.into(); 
        inode.inode().write_at(idx, rec.as_bytes());
        idx += rec.len();
    }
    Ok(())
}






