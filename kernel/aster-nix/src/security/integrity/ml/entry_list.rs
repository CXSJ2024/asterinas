use alloc::{collections::BTreeMap, vec::Vec};

use spin::{Mutex, MutexGuard};

use aster_frame::{arch::console::print, ima::tpm::{PcrValue, DEFAULT_PCR_REGISTER, PCR_BITSIZE}};


use super::entry::MeasurementEntry;

// use ram tpm
use aster_frame::ima::tpm::{ram_tpm::RAMTpm,default_extended_alg,PcrOp};
static PCR_DEVICE: Mutex<RAMTpm> = Mutex::new(RAMTpm {});



static IMA_MEASUREMENT_LIST: Mutex<MeasurementList> = Mutex::new(MeasurementList::default());



pub struct MeasurementList {
    pub version: u8,                        // magic value = 1
    pub appraise: u8,                       // 0:disable ima, 1:fix mode.
    pub policy: u8,                         // default 1 for all files.
    pub template: u8,                       // entry format template, default is '1:ima'.
    inner: BTreeMap<u64, MeasurementEntry>, // entry
}


struct PCR{}

impl PCR {
    pub fn op() -> MutexGuard<'static, RAMTpm> {
        PCR_DEVICE.lock()
    }
}



impl MeasurementList {
    const fn default() -> Self {
        MeasurementList {
            version: 1,
            appraise: 1,
            policy: 1,
            template: 1,
            inner: BTreeMap::new(),
        }
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
        PCR::op().extend_pcr(DEFAULT_PCR_REGISTER,entry.template_hash);
        let entry_id = self.inner.len() + 1;
        self.inner.insert(entry_id as u64, entry);
    }

    pub fn reset_tpm() {
        PCR::op().read_pcr(DEFAULT_PCR_REGISTER);
    }

    pub fn verify_ml(&self) -> bool {
        let entries = self.get_all();
        let mut tmp_data: PcrValue = [0; PCR_BITSIZE];
        for entry in entries {
            tmp_data = default_extended_alg(tmp_data, entry.template_hash);
        }
        let expect = PCR::op().read_pcr(DEFAULT_PCR_REGISTER);
        tmp_data == expect
    }

}



