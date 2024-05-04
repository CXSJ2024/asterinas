use alloc::string::ToString;

use crate::{early_println, ima::entry_list::MeasurementList};

use self::entry::MeasurementEntry;


pub mod entry_list;
pub mod entry;
pub mod tpm;



pub fn test(){
    test_measurement_entry();
}




pub fn test_measurement_entry(){
    MeasurementList::reset_tpm();
    let mut ml = MeasurementList::get_list();
    // test data
    let entry_offest = 0x8;
    let entry_data1 = MeasurementEntry{
        pcr: 10,
        template_hash: [0xa;20],
        filedata_hash: [0xb;40],
        filename_hint: "/regression/hello_world/hello_world".to_string(),
        field: 0,
    };
    let entry_data2 = MeasurementEntry{
        pcr: 10,
        template_hash: [0xc;20],
        filedata_hash: [0xd;40],
        filename_hint: "/regression/hello_world/hello_world".to_string(),
        field: 0,
    };
    let entry_data3 = MeasurementEntry{
        pcr: 10,
        template_hash: [0xe;20],
        filedata_hash: [0xf;40],
        filename_hint: "/regression/hello_world/hello_world".to_string(),
        field: 0,
    };
    ml.add_entry(entry_data1);
    ml.add_entry(entry_data2);
    ml.add_entry(entry_data3);
    early_println!("measurement list content:{:?}",ml.get_all());
    early_println!("measurement list integrity:{}",ml.vertify_tpm());
}



