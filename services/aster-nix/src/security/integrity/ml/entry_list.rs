use alloc::{boxed::Box, collections::BTreeMap, vec::Vec};
use spin::{Mutex, MutexGuard};



use crate::security::integrity::{ima::ima_hash::IMAAlogrithm, pcr::TdxRTMR, IMA_FEATURE_MODE};

use super::entry::MeasurementEntry;


use crate::security::integrity::pcr::{DEFAULT_PCR_REGISTER, PCR_BITSIZE,PcrValue,PcrOp};




static IMA_MEASUREMENT_LIST: Mutex<MeasurementList> = Mutex::new(MeasurementList::default());

// path prefix need to measure in fix mode
pub const FIX_MODE_PREFIX: [&str;3] = ["/etc","/usr","/regression"];


pub fn check_hint(abs_path:&str,ml_appraise:u8) -> bool{
    match ml_appraise {
        1 => {
            for s in FIX_MODE_PREFIX.iter(){
                if let Some(a) = abs_path.find(s){
                    if a == 0 {
                        return true;
                    }
                }
            }
            false
        },
        _ => {
            false
        },
    }
}


pub fn select_ima_algo(ml_template:u8) -> Option<IMAAlogrithm>{
    match ml_template {
        _ => Some(IMAAlogrithm::default())
    }
}

#[derive(PartialEq, Eq)]
pub enum PCR{
    Ram,
    TpmChip,
    Tdx,
} 

impl PCR {

    pub fn dev_type() -> Self{
        Self::Tdx
    }

    pub fn op() -> Box<dyn PcrOp> {
        Box::new(TdxRTMR {})
    }

    pub fn has_pcr() -> bool{
        if Self::dev_type() == Self::Ram{
            true
        }else{
            false
        }
    }
}


pub struct MeasurementList {
    pub version: u8,                        // magic value = 1
    pub appraise: u8,                       // 1:fix mode.
    pub policy: u8,                         // 
    pub template: u8,                       // algo for entry format template
    inner: BTreeMap<u64, MeasurementEntry>, // entry
}

impl MeasurementList {
    const fn default() -> Self {
        MeasurementList {
            version: 1,
            appraise: IMA_FEATURE_MODE,
            policy: 1,
            template: 1,
            inner: BTreeMap::new(),
        }
    }

    pub fn ima_feature_on(&self) -> bool{
        self.appraise > 0
    }

    pub fn get_list() -> MutexGuard<'static, Self> {
        IMA_MEASUREMENT_LIST.lock()
    }

    
    pub fn get_all(&self) -> Vec<MeasurementEntry> {
        self.inner.values().cloned().collect()
    }

    pub fn get_entry(&self, id: u64) -> Option<&MeasurementEntry> {
        self.inner.get(&id)
    }

    // tpm operation
    pub fn add_entry(&mut self, entry: MeasurementEntry) {
        if PCR::has_pcr(){
            let extended_data = PCR::op().replay_algo(
                PCR::op().read_pcr(DEFAULT_PCR_REGISTER), 
                entry.template_hash
            );
            PCR::op().extend_pcr(DEFAULT_PCR_REGISTER,extended_data);
        }
        let entry_id = self.inner.len() + 1;
        self.inner.insert(entry_id as u64, entry);
    }


    pub fn verify_ml(&self) -> bool {
        if !PCR::has_pcr() {
            return true;
        }
        let entries = self.get_all();
        let mut tmp_data: PcrValue = [0; PCR_BITSIZE];
        for entry in entries {
            tmp_data = PCR::op().replay_algo(tmp_data,entry.template_hash);
        }
        let expect = PCR::op().read_pcr(DEFAULT_PCR_REGISTER);
        tmp_data == expect
    }

}



