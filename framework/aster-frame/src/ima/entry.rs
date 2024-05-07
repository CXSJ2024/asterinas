use core::fmt::Display;


use alloc::string::String;



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



