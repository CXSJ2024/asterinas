use core::fmt::Display;


use alloc::{format, string::{String, ToString}, vec::Vec};
use crate::security::integrity::pcr::{DEFAULT_PCR_REGISTER, PCR_BITSIZE,PcrValue};

use crate::security::integrity::ima::ima_hash::{IMAHash, VecU8,IMAAlogrithm};




const HASH_DATA_SIZE:usize = PCR_BITSIZE;

#[derive(Debug, Eq, PartialEq, Default, Clone)]
pub enum EntryTemplate{
    #[default]
    ImaNg, //0x10
    Ima,   //0x20
    Unknown
}

impl EntryTemplate {
    pub fn template_from_u32(field:u32)->Self{
        match field & 0xf0 {
            0x10 => Self::ImaNg,
            0x20 => Self::Ima,
            _ => Self::Unknown
        }
    }

    pub fn algo_from_u32(field:u32)->IMAAlogrithm{
        match field & 0x0f {
            0x1 => IMAAlogrithm::SHA384,
            0x2 => IMAAlogrithm::SHA256,
            _ => IMAAlogrithm::SHA384,
        }
    } 
}

impl Into<String> for EntryTemplate{
    fn into(self) -> String {
        match self {
            Self::ImaNg => String::from("ima-ng"),
            Self::Ima => String::from("ima"),
            Self::Unknown => String::from("unknown")
        }
    }
}




#[derive(Clone,Debug)]
pub struct MeasurementEntry{
    pub pcr: u8,                                // pcr no.
    pub template_hash: PcrValue,                // data to extend in pcr register
    pub filedata_hash: [u8;HASH_DATA_SIZE],     // file hash data
    pub filename_hint: String,                  // file path name
    pub field:u32                               // 0x10: ima-ng, 0x01:sha1
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

impl Into<String> for MeasurementEntry{
    fn into(self) -> String {
        let template_str = VecU8::new(self.template_hash.to_vec()).to_string();
        let template_type:String = EntryTemplate::template_from_u32(self.field).into();
        let content_str: String = IMAHash{ 
            algo: EntryTemplate::algo_from_u32(self.field), 
            hash: VecU8::new(self.filedata_hash.to_vec()) 
        }.into();
        format!("{} {} {} {} {}\n",
            self.pcr,
            template_str,
            template_type,
            content_str,
            self.filename_hint
        )
    }
}

impl MeasurementEntry{
    pub fn new(hint:&str,template_data:&Vec<u8>, filedata:&Vec<u8>) -> Self{
        let mut template = [0;PCR_BITSIZE];
        let mut content = [0;HASH_DATA_SIZE];
        template[..template_data.len()].clone_from_slice(&template_data[..]);
        content[..template_data.len()].clone_from_slice(&filedata[..]);
        MeasurementEntry{ 
            pcr: DEFAULT_PCR_REGISTER, 
            template_hash: template, 
            filedata_hash: content, 
            filename_hint: hint.to_string(), 
            field: 0x11 
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


