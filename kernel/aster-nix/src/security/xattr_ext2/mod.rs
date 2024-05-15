use alloc::format;
use alloc::string::ToString;
use alloc::{sync::Arc, vec::Vec};
use crate::fs::fs_resolver::{FsResolver,FsPath};
use crate::security::xattr_ext2::getfattr::get_xattr;
use crate::security::xattr_ext2::listfattr::list_attr;
use crate::{fs::utils::InodeMode, println};
use crate::fs::utils::Inode;
use crate::String;
use crate::Errno;

use self::setfattr::set_xattr;


pub mod getfattr;
pub mod setfattr;
pub mod listfattr;

pub struct Xattr{
    pub xattr_block: Arc<dyn Inode>,
}

#[derive(Debug)]
pub struct XattrEntry{
    pub attribute: String,
    pub value: String,
    pub file_ino: u64,
}

const XATTR_PATH :&str = "/xattr";

const USER_EXECUTABLE_PREFIX :&str = "/regression"; 

const IMA_XATTR_KEY :&str = "security.ima";

impl Xattr{
    pub fn xattr_handler() -> Result<Self,Errno>{
        let resolver = FsResolver::new();
        if let Ok(path) = FsPath::new(0, XATTR_PATH){
            let dentry = resolver.lookup(&path).unwrap();
            Ok(Xattr{
                xattr_block: dentry.inode().clone(),
            })
        }else{
            Err(Errno::EBADF)
        }
    }
}

fn encode_xattr_entry(xattr: &XattrEntry) -> Vec<u8>{
    format!("{}|{}|{}\n",xattr.attribute,xattr.value,xattr.file_ino).as_bytes().to_vec()
}

fn decode_xattr_entry(data: &mut Vec<u8>) -> Result<XattrEntry,Errno>{
    let str = String::from_utf8_lossy(&data[..]);
    let terms:Vec<&str> = str.split("|").collect();
    if terms.len() == 3 {
        let num = terms[2].parse::<u64>();
        if num.is_err(){
            return  Err(Errno::EBADF);
        }
        Ok(XattrEntry{
            attribute: terms[0].to_string(),
            value: terms[1].to_string(),
            file_ino: num.unwrap(),
        })
    }else {
        Err(Errno::EBADF)
    }
}



pub fn xattr_init() -> Option<Xattr>{
    let fs_resolver = FsResolver::new();
    let root_inode = fs_resolver.root().inode();
    let xattr_inode = root_inode.create(&XATTR_PATH[1..], crate::fs::utils::InodeType::File, InodeMode::all());
    if let (Ok(inode),Ok(_)) = (xattr_inode,
        measure_all(&fs_resolver,USER_EXECUTABLE_PREFIX)){
        let _ = Xattr::xattr_handler(); 
        println!("[kernel] File extended attribute in inode #{}",inode.metadata().ino);
        Some(Xattr{
            xattr_block: inode,
        })
    }else{
        println!("[kernel] Securrity Extended File Attribute init fail");
        None
    }
}

fn measure_all(resolver: &FsResolver,root_dir:&str) -> crate::prelude::Result<()>{
    let fs_handler = {
        let path = FsPath::new(0, root_dir)?;
        resolver.open(&path, 0, 0)?
    };
    if fs_handler.dentry().type_().is_reguler_file(){
        let measurement = "todo()!";
        set_xattr(root_dir, IMA_XATTR_KEY, measurement);
    }else if fs_handler.dentry().type_().is_directory(){
        let mut items:Vec<String> = Vec::new();
        fs_handler.readdir(&mut items);
        for item in items{
            if item == "." || item == ".." {
                continue;
            }
            let abs_path = format!("{}/{}",root_dir,item.to_string());
            measure_all(resolver,abs_path.as_str())?;
        }
    }
    Ok(())
}

pub fn test(){
    // user executable files
    let abs_path1 = "/regression/hello_world/hello_world";
    let abs_path2 = "/regression/execve/hello";

    // empty before set_xattr
    println!("Attributes is empty?  {}",list_attr(abs_path1).is_err());

    // set xattr
    set_xattr(abs_path1, "deedbeef.field","invalue attribute");
    set_xattr(abs_path1, "user.field","user value");
    set_xattr(abs_path1, "security.ima","file1_hash_data1");
    set_xattr(abs_path1, "security.ima","file1_hash_data2");
    set_xattr(abs_path2, "security.ima","file2_hash_data");

    // test result
    println!("List attributes:{:?}",list_attr(abs_path1).unwrap());
    println!("Get security.ima attribute:{:?}",get_xattr(abs_path1, "security.ima").unwrap().value);
}


