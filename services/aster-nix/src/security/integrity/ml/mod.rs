use alloc::string::String;
use entry::MeasurementEntry;
use entry_list::PCR;

use crate::{
    fs::{
        fs_resolver::{FsPath, FsResolver},
        utils::InodeMode,
    }, Result
};

use self::entry_list::MeasurementList;

use spin::MutexGuard;

use super::pcr::{DEFAULT_PCR_REGISTER, PCR_BITSIZE};
use alloc::boxed::Box;
use digest::DynDigest;
use sha2::Sha384;


pub mod entry_list;
pub mod entry;


pub const MEASUREMENT_LIST_ASCII: &str = "/ascii_runtime_measurements";

pub fn measurement_list_init() -> Result<()>{
    let fs_resolver = FsResolver::new();
    let root_inode = fs_resolver.root().inode();
    let ml_inode = root_inode.create(
        &MEASUREMENT_LIST_ASCII[1..],
        crate::fs::utils::InodeType::File,
        InodeMode::all(),
    )?;
    if PCR::has_pcr(){
        let original_val = PCR::op().read_pcr(DEFAULT_PCR_REGISTER);
        let mut hasher:Box<dyn DynDigest> = Box::new(Sha384::default());
        let template_val = match PCR::dev_type(){
            PCR::Ram => todo!(),
            PCR::TpmChip => todo!(),
            PCR::Tdx => {
                [original_val.to_vec(),"/dev/tdx_guest".as_bytes().to_vec()].concat()
            },
        };
        hasher.update(&template_val[..]);
        let mut res = [0 as u8; PCR_BITSIZE];
        res.copy_from_slice(&hasher.finalize().to_vec()[..PCR_BITSIZE]);
        let boot_entry = MeasurementEntry::new("boot_aggregate", &res.to_vec(), &original_val.to_vec());
        MeasurementList::get_list().add_entry(boot_entry);
    }
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
        let _ = inode.inode().write_at(idx, rec.as_bytes());
        idx += rec.len();
    }
    Ok(())
}






