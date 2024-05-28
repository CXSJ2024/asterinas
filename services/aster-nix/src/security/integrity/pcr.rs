use spin::Mutex;

use crate::fs::inode_handle::FileIo;


pub const PCR_BITSIZE: usize = 20;
pub static DEFAULT_PCR_REGISTER: u32 = 10;
pub type PcrValue = [u8; PCR_BITSIZE];
pub trait PcrOp {
    fn read_pcr(&self, reg: u32) -> PcrValue;
    fn extend_pcr(&self, reg: u32, data: PcrValue);
    fn reset_pcr(&self, reg: u32);
    fn reset_all(&self);
}


static TDX_GUEST_DEV: Mutex<crate::device::TdxGuest> = Mutex::new(crate::device::TdxGuest{});
const TDX_PCR_VADDR:usize = 0;


pub struct TdxRTMR{}

impl PcrOp for TdxRTMR{
    fn read_pcr(&self, reg: u32) -> PcrValue {
        todo!();
        let dev = TDX_GUEST_DEV.lock();
        let vaddr = TDX_PCR_VADDR;
        let _ = dev.ioctl(crate::fs::utils::IoctlCmd::TDXGETREPORT, vaddr);
    }

    fn extend_pcr(&self, reg: u32, data: PcrValue) {
        todo!();
        let dev = TDX_GUEST_DEV.lock();
        let vaddr = TDX_PCR_VADDR;
        let _ = dev.ioctl(crate::fs::utils::IoctlCmd::TDXEXTENDRTMR, vaddr);
    }

    fn reset_pcr(&self, reg: u32) {
        todo!()
    }

    fn reset_all(&self) {
        todo!()
    }
}
