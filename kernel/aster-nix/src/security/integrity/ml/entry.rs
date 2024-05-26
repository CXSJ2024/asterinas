use core::fmt::Display;


use alloc::string::{String, ToString};
use aster_frame::ima::tpm::{DEFAULT_PCR_REGISTER, PCR_BITSIZE};



const HASH_DATA_SIZE:usize = 40;

#[derive(Clone,Debug)]
pub struct MeasurementEntry{
    pub pcr: u32,                               // pcr no.
    pub template_hash: [u8;20],                 // data to extend in pcr register
    pub filedata_hash: [u8;HASH_DATA_SIZE],     // file hash data
    pub filename_hint: String,                  // file path name
    pub field:u32
}

impl Display for MeasurementEntry{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "pcr:{}\t template-hash:{:x?}\t filedata-hash:{:x?}\t filename-hint:{}]",
        self.pcr, 
        self.template_hash, 
        self.filedata_hash, 
        self.filename_hint
        )
    }
}

impl MeasurementEntry{
    pub fn new(hint:&str) -> Self{
        MeasurementEntry{ 
            pcr: DEFAULT_PCR_REGISTER, 
            template_hash: [0;PCR_BITSIZE], 
            filedata_hash: [0;HASH_DATA_SIZE], 
            filename_hint: hint.to_string(), 
            field: 0 
        }
    }
}

impl Default for MeasurementEntry{
    fn default() -> Self {
        Self { 
            pcr: DEFAULT_PCR_REGISTER, 
            template_hash: [0;PCR_BITSIZE], 
            filedata_hash: [0;HASH_DATA_SIZE], 
            filename_hint: "".to_string(), 
            field: 0 
        }
    }
}


