use alloc::string::ToString;


use crate::println;

use self::entry_list::MeasurementList;

use self::entry::MeasurementEntry;


pub mod entry_list;
pub mod entry;




pub fn measurement_list_init(){
    MeasurementList::reset_tpm();
}




pub fn test_measurement_entry(){
    MeasurementList::reset_tpm();
    let mut ml = MeasurementList::get_list();
    // test data
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
    let ml_data = ml.get_all();
    for e in ml_data{
        println!("{}",e);
    }
    println!("measurement list integrity:{}",ml.verify_ml());
}




