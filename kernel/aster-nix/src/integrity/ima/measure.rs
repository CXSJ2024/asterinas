use alloc::format;

use sha2::{Digest, Sha256};

use crate::{
    fs::{
        file_handle::FileLike,
        file_table::FileDesc,
        fs_resolver::{FsPath, FsResolver, AT_FDCWD},
        utils::{InodeType, SeekFrom},
    },
    prelude::*,
};

pub fn ima_measure(fd: FileDesc, buf_len: usize) -> Result<()> {
    let current = current!();
    let fs = current.fs().read();
    let dentry = fs.lookup_from_fd(fd)?;
    let abs_path = dentry.abs_path();
    let inode = dentry.inode();
    let mut hasher = Sha256::new();
    let mut read_buf = vec![0u8; buf_len];
    let mut pos: usize = 0;
    loop {
        let read_len = inode.read_at(pos, &mut read_buf)?;
        hasher.update(&read_buf[..read_len]);
        pos += read_len;
        if read_len < buf_len {
            break;
        }
    }
    let data_hash = hasher.finalize();
    hasher = Sha256::new();
    hasher.update(&data_hash);
    hasher.update(data_hash.len().to_le_bytes());
    hasher.update(&abs_path);
    hasher.update(abs_path.len().to_le_bytes());
    let template_hash = hasher.finalize();
    let hash_to_hex =
        |bytes: &[u8]| -> String { bytes.iter().map(|byte| format!("{:02x}", byte)).collect() };
    if abs_path.ends_with("txt") {
        println!(
            "=data_hash: expect {}, found {}=",
            "4dc19d3e28cff87c219dd8bd486decbe7407920b9d7739fcc630079fd2a9e01b",
            hash_to_hex(&data_hash)
        );
        println!(
            "=template_hash: expect {}, found {}=",
            "ae9b7d5f9bfc99e64589722495cbb2fc2e0f6061c6ae30d69f6ff3445040913f",
            hash_to_hex(&template_hash)
        );
        if hash_to_hex(&data_hash)
            == "4dc19d3e28cff87c219dd8bd486decbe7407920b9d7739fcc630079fd2a9e01b".to_string()
            && hash_to_hex(&template_hash)
                == "ae9b7d5f9bfc99e64589722495cbb2fc2e0f6061c6ae30d69f6ff3445040913f".to_string()
        {
            println!("measurement passed");
        } else {
            println!("measurement failed");
            return Err(Error::new(Errno::EINVAL));
        }
    }
    Ok(())
}

pub fn process_measurement(fs: &FsResolver, pathname: &str, buf_len: usize) -> Result<()> {
    let mode: u16 = 0;
    let flags: u32 = 0;
    let file_handle = {
        let fs_path = FsPath::new(AT_FDCWD, pathname)?;
        let mask_mode = mode;
        fs.open(&fs_path, flags, mask_mode)?
    };
    if file_handle.dentry().type_() == InodeType::Dir {
        return Ok(());
    }
    let mut read_buf = vec![0u8; buf_len];
    let mut hasher = Sha256::new();
    let mut pos: usize = 0;
    loop {
        file_handle.seek(SeekFrom::Start(pos))?;
        let read_len = file_handle.read(&mut read_buf)?;
        hasher.update(&read_buf[..read_len]);
        pos += read_len;
        if read_len < buf_len {
            break;
        }
    }
    let data_hash = hasher.finalize();
    hasher = Sha256::new();
    hasher.update(&data_hash);
    hasher.update(data_hash.len().to_le_bytes());
    hasher.update(&pathname);
    hasher.update(pathname.len().to_le_bytes());
    let template_hash = hasher.finalize();
    println!(
        "======================\ntemplate-hash:\n{:02x}\nfiledata-hash:\nsha256:{:02x}\nfilename-hint:\n{}",
        template_hash, data_hash, pathname
    );

    Ok(())
}

pub fn test_measurement() -> Result<Vec<String>> {
    let pathname_str = "/foo";
    let fs = FsResolver::new();
    let mode: u16 = 0;
    let flags: u32 = 0;
    let file_handle = {
        let pathname = pathname_str.to_string();
        let fs_path = FsPath::new(AT_FDCWD, pathname_str)?;
        let mask_mode = mode;
        fs.open(&fs_path, flags, mask_mode)?
    };
    if file_handle.dentry().type_() != InodeType::Dir {
        println!("not a directory");
    }
    let mut items: Vec<String> = Vec::new();
    let _ = file_handle.readdir(&mut items)?;
    for file in &items {
        let full_path = format!("{}/{}", pathname_str, file);
        let _ = process_measurement(&fs, &full_path, 1024);
    }
    Ok(items)
}
