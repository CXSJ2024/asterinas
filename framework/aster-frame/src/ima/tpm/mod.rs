use core::hash::BuildHasherDefault;
use spin::{Mutex, MutexGuard};
use ram_tpm::RAMTpm;
use crate::early_println;

pub mod ram_tpm;

pub const PCR_BITSIZE:usize = 20;
pub static DEFAULT_PCR_REGISTER:usize = 10;

pub type PcrValue = [u8;PCR_BITSIZE];
pub trait PcrOp {
    fn read_pcr(&self,reg:u32) -> PcrValue;
    fn extend_pcr(&self,reg:u32,data:PcrValue);
    fn reset_pcr(&self,reg:u32);
    fn reset_all(&self);
} 

static TPM_DEVICE:Mutex<RAMTpm> = Mutex::new(RAMTpm{});

pub struct TPM{
    
}

impl TPM {
    pub fn op() -> MutexGuard<'static,RAMTpm>{
        TPM_DEVICE.lock()
    }
}

// sha(read_pcr(reg)||data)
pub fn default_extended(old_data:PcrValue,new_data:PcrValue) -> PcrValue{
    let mut result = [old_data,new_data].concat();
    //let hash = Sha256::digest(&result[..]);
    let mut res = [0 as u8;PCR_BITSIZE];
    res.copy_from_slice(&result[..PCR_BITSIZE]);
    res
}