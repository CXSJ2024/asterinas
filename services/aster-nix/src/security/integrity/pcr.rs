use crate::fs::{self, fs_resolver::FsPath, inode_handle::FileIo};



/// ref in /regression/apps/generate_tdx_quote/tdx_attest.h
// RTMR[2] used for OS app measurement
// size of extend data: 48
pub const DEFAULT_PCR_REGISTER: u32 = 0x2;
pub const PCR_BITSIZE: usize = 20;


pub type PcrValue = [u8; PCR_BITSIZE];
pub trait PcrOp {
    fn read_pcr(&self, reg: u32) -> PcrValue;
    fn extend_pcr(&self, reg: u32, data: PcrValue);
    fn reset_pcr(&self, reg: u32);
    fn reset_all(&self);
}




pub struct TdxRTMR{}

impl PcrOp for TdxRTMR{
    fn read_pcr(&self, reg: u32) -> PcrValue {
        todo!();
        // possible usage i guess
        // let dev = {
        //     let resolver = fs::fs_resolver::FsResolver::new();
        //     let dev_inode = resolver.lookup(FsPath::new(0, "/dev/tdx_guest")).unwrap().inode();
        //     dev_inode
        // };
        // let vaddr = 0xFFFF_FFFF_FFFF_FFFF;
        // let _ = dev.ioctl(crate::fs::utils::IoctlCmd::TDXGETREPORT, vaddr);
    }

    fn extend_pcr(&self, reg: u32, data: PcrValue) {
        todo!();
    }

    fn reset_pcr(&self, reg: u32) {
        todo!()
    }

    fn reset_all(&self) {
        todo!()
    }
}
