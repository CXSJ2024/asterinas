use crate::fs::{self, fs_resolver::FsPath, inode_handle::FileIo};
use crate::device::tdxguest::{self, *};
use crate::println;
use alloc::boxed::Box;
use digest::DynDigest;
use sha2::Sha384;

/// ref in /regression/apps/generate_tdx_quote/tdx_attest.h
// RTMR[2] used for OS app measurement
// size of extend data: 48
pub const DEFAULT_PCR_REGISTER: u8 = 0x2;
pub const PCR_BITSIZE: usize = 48;


pub type PcrValue = [u8; PCR_BITSIZE];
pub trait PcrOp {
    fn read_pcr(&self, reg: u8) -> PcrValue;
    fn extend_pcr(&self, reg: u8, data: PcrValue);
    fn replay_algo(&self,old:PcrValue,new:PcrValue) -> PcrValue;
}


pub struct TdxRTMR{}

impl PcrOp for TdxRTMR{
    fn read_pcr(&self, reg: u8) -> PcrValue {
        let reportdata = [0;TDX_REPORTDATA_LEN];
        let tdreport = [0;TDX_REPORT_LEN];
        let req = TdxReportReq{ reportdata, tdreport };
        let mut res:PcrValue = [0;PCR_BITSIZE];
        if let Ok(wapper) = tdxguest::kernel_handle_get_report(req){
            let tdreport_info_offest:usize = 0x200;
            let tdreport_info_rtmrs_offest:usize = 0xd0;
            let tdreport_info_rtmrs_len:usize = 4*TDX_EXTEND_RTMR_DATA_LEN;
            let target_offest = tdreport_info_offest + tdreport_info_rtmrs_offest + reg as usize * TDX_EXTEND_RTMR_DATA_LEN;
            let target_rtmr = &wapper.tdx_report[target_offest..target_offest+TDX_EXTEND_RTMR_DATA_LEN];
            res.copy_from_slice(&target_rtmr[..PCR_BITSIZE]);   
        }
        res
    }

    fn extend_pcr(&self, reg: u8, data: PcrValue) {
        let mut tmp = [0;TDX_EXTEND_RTMR_DATA_LEN];
        tmp[..PCR_BITSIZE].copy_from_slice(&data[..]);
        let _ = tdxguest::kernel_handle_extend_rtmr(ExtendRtmrWapper { data: tmp, index: reg });
    }
    
    // see https://github.com/intel/tdx-tools/blob/14a6a4c2e8d466a3bda738da294703c239e5eabc/attestation/pytdxmeasure/pytdxmeasure/actor.py#L104
    fn replay_algo(&self,old:PcrValue,new:PcrValue) -> PcrValue {
        let tmp = [old, new].concat();
        let mut hasher:Box<dyn DynDigest> = Box::new(Sha384::default());
        hasher.update(&tmp[..]);
        let mut res = [0 as u8; PCR_BITSIZE];
        res.copy_from_slice(&hasher.finalize().to_vec()[..PCR_BITSIZE]);
        res
    }
}
