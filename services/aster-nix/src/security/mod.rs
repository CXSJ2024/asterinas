use alloc::string::String;
use integrity::pcr::{TdxRTMR,PcrOp};

use crate::{print, println};



pub mod integrity;
pub mod xattr_ext2;

pub fn init() {
    //test_tdx();
    if integrity::IMA_FEATURE_MODE > 0{
        let _ = integrity::ml::measurement_list_init();
        xattr_ext2::xattr_init();
        //println_ml();
    }
}

fn println_ml(){
    let entries = integrity::ml::entry_list::MeasurementList::get_list().get_all();
    for e in entries{
        let s:String = e.into();
        print!("{}",s);
    }
}

fn test_tdx(){
    let mut data:[u8;48] = [0;48];
    for i in 0..20{
        data[i] = i as u8 + 10;
    }
    let pcr_dev = TdxRTMR{};
    pcr_dev.extend_pcr(2, data.clone());

    let ref_val = pcr_dev.read_pcr(2);
    println!("test_tdx: ref value = {:x?}",ref_val);

    let mut replay_val:[u8;48] = [0;48];
    let replay_val = pcr_dev.replay_algo(replay_val, data.clone());
    println!("test_tdx: replay value = {:x?}",replay_val);
    
    assert_eq!(ref_val,replay_val);
}