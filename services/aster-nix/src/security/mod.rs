use alloc::string::String;

use crate::print;



pub mod integrity;
pub mod xattr_ext2;

pub fn init() {
    if integrity::IMA_FEATURE_MODE > 0{
        let _ = integrity::ml::measurement_list_init();
        xattr_ext2::xattr_init();
        println_ml();
    }
}

fn println_ml(){
    let entries = integrity::ml::entry_list::MeasurementList::get_list().get_all();
    for e in entries{
        let s:String = e.into();
        print!("{}",s);
    }
}
