

pub mod xattr_ext2;

pub fn security_init(){
    xattr_ext2::xattr_init();
    xattr_ext2::test();
}