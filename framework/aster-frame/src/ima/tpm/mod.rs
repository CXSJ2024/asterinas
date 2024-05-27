pub mod ram_tpm;

pub const PCR_BITSIZE: usize = 20;
pub static DEFAULT_PCR_REGISTER: u32 = 10;
pub type PcrValue = [u8; PCR_BITSIZE];
pub trait PcrOp {
    fn read_pcr(&self, reg: u32) -> PcrValue;
    fn extend_pcr(&self, reg: u32, data: PcrValue);
    fn reset_pcr(&self, reg: u32);
    fn reset_all(&self);
}


