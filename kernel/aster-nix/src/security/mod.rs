pub mod integrity;
pub mod xattr_ext2;

pub fn security_init() {
    integrity::ml::measurement_list_init();
    xattr_ext2::xattr_init();
    
}

