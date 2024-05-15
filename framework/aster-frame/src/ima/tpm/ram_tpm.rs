use alloc::vec::Vec;
use pod::Pod;
use crate::early_print;

use super::{PCR_BITSIZE,PcrValue,DEFAULT_PCR_REGISTER,PcrOp,default_extended};



type AlignedPcrValue = [u8;24];

pub struct RAMTpm{
    
}

impl PcrOp for RAMTpm {
    fn read_pcr(&self,reg:u32) -> PcrValue {
        let base_addr = ima_begin() + reg as usize * PCR_BITSIZE;
        copy_bytes_to_fixed_array::<PCR_BITSIZE>(&read_ima(base_addr, 24),0,PCR_BITSIZE).unwrap()
    }

    fn extend_pcr(&self,reg:u32,data:PcrValue) {
        let base_addr = ima_begin() + reg as usize * PCR_BITSIZE;
        let old_data = self.read_pcr(reg);
        write_ima(&align_data(default_extended(old_data,data)).to_vec(), base_addr);
    }

    fn reset_pcr(&self,reg:u32) {
        let base_addr = ima_begin() + reg as usize * PCR_BITSIZE;
        write_ima(&align_data([0;PCR_BITSIZE]).to_vec(), base_addr)
    }

    fn reset_all(&self) {
        self.reset_pcr(DEFAULT_PCR_REGISTER as u32);
    }
}



fn align_data(data:PcrValue)-> AlignedPcrValue{
    let mut res:AlignedPcrValue = [0;24];
    res[0..PCR_BITSIZE].copy_from_slice(&data[..]);
    res
}

fn read_ima(base_addr:usize, len: usize) -> Vec<u8>{
    let mut res:Vec<u8> = Vec::new();
    let step = 8;
    for i in 0..len/step{
        unsafe{
            let bytes = ((base_addr + step*i) as *const u64 ).read();
            res.append(&mut bytes.as_bytes().to_vec());
        }
    }
    res
}

fn write_ima(data :&Vec<u8>, base_addr:usize){
    let step = 8;
    for i in 0..data.len()/step{
        let mut tmp:u64 = 0;
        for j in 0..step{
            tmp += (data[i*step+j] as u64) << step*j;
        }
        unsafe{
            ((base_addr + step*i) as *mut u64 ).write(tmp);
        }
    }
}


fn copy_bytes_to_fixed_array<const N: usize>(bytes: &Vec<u8>, start: usize, end: usize) 
    -> Option<[u8; N]> {
    if start >= bytes.len() || end > bytes.len() || start > end || (end - start) != N {
        return None;
    }
    let mut array: [u8; N] = [0; N]; 
    array.copy_from_slice(&bytes[start..end]); 
    Some(array)
}



fn ima_begin() -> usize {
    extern "C" {
        fn __ima();
    }
    return __ima as usize;
}