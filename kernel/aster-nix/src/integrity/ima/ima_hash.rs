use core::fmt::Display;

use digest::DynDigest;
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};

use crate::{fs::utils::Inode, prelude::*};

#[derive(Eq, PartialEq, Default)]
pub enum IMAAlogrithm {
    #[default]
    SHA1,
    SHA256,
    SHA384,
    SHA512,
    MD5,
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

#[derive(Eq, PartialEq, Default)]
pub struct IMAHash {
    algo: IMAAlogrithm,
    hash: Vec<u8>,
}

pub fn select_hasher(algo: &IMAAlogrithm) -> Box<dyn DynDigest> {
    match algo {
        IMAAlogrithm::SHA1 => Box::new(Sha1::default()),
        IMAAlogrithm::SHA256 => Box::new(Sha256::default()),
        IMAAlogrithm::SHA384 => Box::new(Sha384::default()),
        IMAAlogrithm::SHA512 => Box::new(Sha512::default()),
        _ => Box::new(Sha1::default()),
    }
}

pub fn cal_fd_hash(
    inode: &Arc<dyn Inode>,
    buf_len: usize,
    algo: Option<IMAAlogrithm>,
) -> Result<IMAHash> {
    let algo = algo.unwrap_or(IMAAlogrithm::SHA256);
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
    Ok(IMAHash {
        algo,
        hash: hasher.finalize().to_vec(),
    })
}
