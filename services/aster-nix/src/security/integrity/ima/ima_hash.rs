use alloc::format;
use core::{fmt::Display, str::FromStr};

use digest::DynDigest;
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};

use crate::{fs::utils::Inode, prelude::*};

#[derive(Debug, Eq, PartialEq, Default, Clone)]
pub enum IMAAlogrithm {
    SHA1,
    SHA256,
    #[default]
    SHA384,
    SHA512,
    MD5,
}

#[derive(Eq, Debug, PartialEq, Default)]
pub struct IMAHash {
    pub algo: IMAAlogrithm,
    pub hash: VecU8,
}

#[derive(Eq, PartialEq, Default)]
pub struct VecU8 {
    pub data: Vec<u8>,
}

impl Display for IMAAlogrithm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            IMAAlogrithm::SHA1 => write!(f, "SHA1"),
            IMAAlogrithm::SHA256 => write!(f, "SHA256"),
            IMAAlogrithm::SHA384 => write!(f, "SHA384"),
            IMAAlogrithm::SHA512 => write!(f, "SHA512"),
            IMAAlogrithm::MD5 => write!(f, "MD5"),
        }
    }
}

impl FromStr for IMAAlogrithm {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "SHA1" => Ok(IMAAlogrithm::SHA1),
            "SHA256" => Ok(IMAAlogrithm::SHA256),
            "SHA384" => Ok(IMAAlogrithm::SHA384),
            "SHA512" => Ok(IMAAlogrithm::SHA512),
            "MD5" => Ok(IMAAlogrithm::MD5),
            _ => Err(Error::new(Errno::ENAVAIL)),
        }
    }
}

impl Into<String> for IMAHash {
    fn into(self) -> String {
        format!("{}:{}", self.algo, self.hash)
    }
}

impl FromStr for IMAHash {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(':');
        let algo = parts.next().unwrap().parse()?;
        let hash = parts.next().unwrap().parse()?;
        Ok(IMAHash { algo, hash })
    }
}

impl From<String> for IMAHash {
    fn from(s: String) -> Self {
        let mut parts = s.split(':');
        let algo = parts.next().unwrap().parse().unwrap();
        let hash = parts.next().unwrap().parse().unwrap();
        IMAHash { algo, hash }
    }
}

impl Debug for VecU8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for byte in self.data.iter() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}
impl FromStr for VecU8 {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut data = Vec::new();
        for i in 0..s.len() / 2 {
            match u8::from_str_radix(&s[i * 2..i * 2 + 2], 16) {
                Ok(byte) => data.push(byte),
                Err(_) => return Err(Error::new(Errno::EINVAL)),
            }
        }
        Ok(VecU8 { data })
    }
}

impl Display for VecU8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for byte in self.data.iter() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl VecU8 {
    pub fn new(data: Vec<u8>) -> Self {
        VecU8 { data }
    }
}

pub fn select_hasher(algo: &IMAAlogrithm) -> Box<dyn DynDigest> {
    match algo {
        IMAAlogrithm::SHA1 => Box::new(Sha1::default()),
        IMAAlogrithm::SHA256 => Box::new(Sha256::default()),
        IMAAlogrithm::SHA384 => Box::new(Sha384::default()),
        IMAAlogrithm::SHA512 => Box::new(Sha512::default()),
        _ => Box::new(Sha384::default()),
    }
}

pub fn cal_fd_hash(
    inode: &Arc<dyn Inode>,
    buf_len: usize,
    algo: Option<IMAAlogrithm>,
    path: Option<&str>
) -> Result<IMAHash> {
    let algo = algo.unwrap_or(IMAAlogrithm::default());
    let mut hasher = select_hasher(&algo);
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
    if let Some(s) = path{
        hasher.update(s.as_bytes());
    }
    Ok(IMAHash {
        algo,
        hash: VecU8::new(hasher.finalize().to_vec()),
    })
}
