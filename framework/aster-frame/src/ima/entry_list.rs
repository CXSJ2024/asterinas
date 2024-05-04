
use alloc::{collections::BTreeMap, vec::Vec};
use spin::{Mutex, MutexGuard};


use super::{entry::MeasurementEntry, tpm::{self, PcrOp, TPM}};

static IMA_MEASUREMENT_LIST:Mutex<MeasurementList> = Mutex::new(MeasurementList::default());

pub struct MeasurementList{
    pub version: u8,            // magic value = 1
    pub appraise: u8,           // 0:disable ima, 1:fix mode.
    pub policy: u8,             // default 1 for all files.
    pub template: u8,           // entry format template, default is '1:ima'.
    inner: BTreeMap<u64, MeasurementEntry> // entry
}






impl MeasurementList {
    const fn default() -> Self{
        MeasurementList{
            version: 1,
            appraise: 1,
            policy: 1,
            template: 1,
            inner: BTreeMap::new(),
        }
    }

    pub fn get_list() -> MutexGuard<'static,Self>{
        IMA_MEASUREMENT_LIST.lock()
    }

    pub fn reset_tpm(){
        TPM::op().reset_all();
    }

    pub fn get_all(&self) -> Vec<MeasurementEntry>{
        self.inner.values().cloned().collect()
    }

    pub fn get_entry(&self,id:u64) -> Option<&MeasurementEntry>{
        self.inner.get(&id)
    }

    pub fn add_entry(&mut self,entry:MeasurementEntry){
        TPM::op().extend_pcr(tpm::DEFAULT_PCR_REGISTER as u32, entry.template_hash);
        let entry_id = self.inner.len() + 1;
        self.inner.insert(entry_id as u64, entry);
    }


    pub fn vertify_tpm(&self) -> bool{
        let entries = self.get_all();
        let mut tmp_data:tpm::PcrValue = [0;tpm::PCR_BITSIZE];
        for entry in entries{
            tmp_data = tpm::default_extended(tmp_data, entry.template_hash);
        }
        
        tmp_data == TPM::op().read_pcr(tpm::DEFAULT_PCR_REGISTER as u32)
    }
}





