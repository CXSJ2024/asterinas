
use alloc::vec::Vec;

use super::ima_begin;

pub struct MeasurementList{
    pub entry_base_addr: u64,   // .ima section base addrest
    pub version: u8,            // magic version = 1
    pub appraise: u8,           // 0 for disable ima, 1 for fix mode.
    pub policy: u8,             // 1 for all executable files.
    pub template: u8,           // entry format template, 1 for 'ima'.
    pub entries: Vec<EntryMap>
}





impl MeasurementList {
    pub fn default() -> Self{
        MeasurementList{
            entry_base_addr:ima_begin() as u64,
            version: 1,
            appraise: 1,
            policy: 1,
            template: 1,
            entries: Vec::new(),
        }
    }



    pub fn find_entry(&self,id:u32) -> Option<u32>{
        if self.entries.is_empty() {
            return None;
        }
        let magic_entry = self.entries.get(0).unwrap();
        if magic_entry.id == id {
            Some(magic_entry.offset)
        } else{
            None
        }
    }


}



pub struct EntryMap{
    pub id:u32,                 // entry id, store in fattr
    pub offset:u32,             // entry address (entry_base_addr + offset) 
}


impl EntryMap {
    pub fn new(id: u32, offset:u32) -> Self{
        EntryMap{
            id,
            offset,
        }
    }
}