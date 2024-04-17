use core::fmt::Display;

use alloc::{string::String, vec::Vec};
use pod::Pod;

use crate::early_println;

use super::{entry_list::MeasurementList, read_ima, write_ima};

pub const HASH_DATA_SIZE:usize = 48;
pub const PATH_SIZE:usize = 256;

pub struct MeasurementEntry{
    pub length: u32,
    pub hash_data: [u8;HASH_DATA_SIZE],     // 48 bytes hash data
    pub path: [u8;PATH_SIZE],               // 255 bytes file name
    pub fields: u64                         // reserved fields
}

impl Display for MeasurementEntry{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Measurement Entry: [\n\t file:{},\n\t hash value:{},\n\t fields:{},\n]",
        String::from_utf8_lossy(&self.path), 
        String::from_utf8_lossy(&self.hash_data), 
        String::from_utf8_lossy(&self.fields.as_bytes()), 
        )
    }
}

fn concat_bytes_to_u32(bytes: &Vec<u8>) -> Option<u32> {
    if bytes.len() != 4 {
        return None; 
    }
    let mut buf = [0u8; 4];
    buf.copy_from_slice(&bytes[..4]);
    Some(u32::from_be_bytes(buf))
}

fn concat_bytes_to_u64(bytes: &Vec<u8>) -> Option<u64> {
    if bytes.len() != 8 {
        return None; 
    }
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&bytes[..8]);
    Some(u64::from_be_bytes(buf))
}

fn copy_bytes_to_fixed_array<const N: usize>(bytes: &Vec<u8>, start: usize, end: usize) 
    -> Option<[u8; N]> {
    if start >= bytes.len() || end > bytes.len() || start > end || (end - start) != N {
        return None;
    }
    let mut array: [u8; N] = [0; N]; 
    array.copy_from_slice(&bytes[start..end]); 
    Some(array)
}

pub fn read_entry(entry_list: &MeasurementList, entry_id: u32) -> Option<MeasurementEntry>{
    if let Some(offset) = entry_list.find_entry(entry_id){
        early_println!("find entry:0x{:x} with offset:0x{:x}",entry_id, offset);
        let addr = entry_list.entry_base_addr as usize + offset as usize;
        //let entry_size = concat_bytes_to_u32(&read_ima(addr, 4));
        let entry_size = Some(0x138 as u32);
        if entry_size.is_none() {
            return None;
        }
        early_println!("entry size:0x{:x}",entry_size.as_ref().unwrap().clone());
        let entry_data = read_ima(addr, entry_size.unwrap() as usize);
        let hash_data = copy_bytes_to_fixed_array::<HASH_DATA_SIZE>(&entry_data,4,4+HASH_DATA_SIZE);
        let path = copy_bytes_to_fixed_array::<PATH_SIZE>(&entry_data,4+HASH_DATA_SIZE,4+HASH_DATA_SIZE+PATH_SIZE);
        let fields = concat_bytes_to_u64(&entry_data[(4+HASH_DATA_SIZE+PATH_SIZE)..].to_vec());
        if hash_data.is_none() || fields.is_none() || path.is_none(){
            return None;
        }
        early_println!("read new entry complete");
        Some(MeasurementEntry{
            length: entry_size.unwrap(),
            hash_data: hash_data.unwrap(),
            path: path.unwrap(),
            fields: fields.unwrap(),
        })
    }else {
        None
    } 
}
    

pub fn write_entry(entry_list: &MeasurementList, entry_id: u32, entry_data: &MeasurementEntry){
    if let Some(offset) = entry_list.find_entry(entry_id){
        early_println!("find entry:0x{:x} with offset:0x{:x}",entry_id, offset);
        let addr = entry_list.entry_base_addr as usize + offset as usize;
        let mut data:Vec<u8> = Vec::new();
        data.append(&mut entry_data.length.as_bytes().to_vec());
        data.append(&mut entry_data.hash_data.as_bytes().to_vec());
        data.append(&mut entry_data.path.as_bytes().to_vec());
        data.append(&mut entry_data.fields.as_bytes().to_vec());
        write_ima(&data, addr);
    }
}

pub struct FileAttr{
    pub id: u32,                            // map to entry id
    pub reference_hash: [u8;HASH_DATA_SIZE] // expected hash data
}