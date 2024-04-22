use alloc::vec::Vec;
use pod::Pod;


use crate::{early_println, ima::entry_list::{EntryMap, MeasurementList}};

use self::entry::{read_entry, write_entry, MeasurementEntry, PATH_SIZE,HASH_DATA_SIZE};


pub mod entry_list;
pub mod entry;

pub fn read_ima(base_addr:usize, len: usize) -> Vec<u8>{
    let mut res:Vec<u8> = Vec::new();
    let step = 8;
    if len < step{
        unsafe{
            let bytes: u32 = (base_addr as *const u64 ).read() as u32;
            //early_println!("0x{:x}",bytes);
            res.append(&mut (0x138 as u32).as_bytes().to_vec());
        }
    }else{
        for i in 0..len/step{
            unsafe{
                let bytes = ((base_addr + step*i) as *const u64 ).read();
                res.append(&mut bytes.as_bytes().to_vec());
            }
        }
    }
    res
}

pub fn write_ima(data :&Vec<u8>, base_addr:usize){
    let step = 8;
    for i in 0..data.len()/step{
        let mut tmp:u64 = 0;
        for j in 0..step{
            tmp += (data[i*step+j] as u64) << step*j;
        }
        //early_println!("0x{:x}",tmp);
        unsafe{
            ((base_addr + step*i) as *mut u64 ).write(tmp);
        }
    }
}

pub fn test(){
    //test_ima_section();
    //test_measurement_entry();
}

// pub fn test_xattr(){
//     let tmp = tempfile_in("/var/tmp").unwrap();
//     assert!(tmp.get_xattr("user.test").unwrap().is_none());
// }

pub fn test_ima_section(){
    let mut data = Vec::new();
    let len = 32;
    for i in 0..len{
        data.push((i+1) as u8);
    }
    let addr = ima_begin() + 8;
    write_ima(&data, addr);
    early_println!("read data:{:?}",read_ima(addr, data.len()));
}

pub fn test_measurement_entry(){
    let mut ml = MeasurementList::default();
    // test data
    let entry_id = 0xbeef;
    let entry_offest = 0x8;
    let mut entry_data = MeasurementEntry{
        length: (4+HASH_DATA_SIZE+PATH_SIZE+4) as u32,
        hash_data: [0;HASH_DATA_SIZE],
        path: [0;PATH_SIZE],
        fields: 0x40400000,
    };
    let entry_map = EntryMap::new(entry_id,entry_offest);
    let hash_str = "this_is_hash_test".as_bytes();
    let path_str = "root/asterinas/regression/apps/test_ami/test".as_bytes();
    entry_data.hash_data[..hash_str.len()].copy_from_slice(hash_str);
    entry_data.path[..path_str.len()].copy_from_slice(path_str);
    ml.entries.push(entry_map);
    early_println!("{}",&entry_data);
    write_entry(&ml, entry_id, &entry_data);
    if let Some(entry) = read_entry(&ml, entry_id){
        early_println!("{}",&entry);
    }
}



pub fn ima_begin() -> usize {
    extern "C" {
        fn __ima();
    }
    return __ima as usize;
}