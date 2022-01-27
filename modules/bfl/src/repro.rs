use std::mem::size_of;

use core::exec::call::Call;
use core::exec::fd_info::Fd;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PocHeader {
    pub len: usize,
    pub cid: u64,
    pub uid: u64,
    pub fid_size: usize,
    pub mem_size: usize,
    pub dmp_size: usize,
}

pub struct PocCall {
    pub info: PocHeader,
    pub fid: Vec<u8>,
    pub mem: Vec<u8>,
    pub dmp: Vec<u8>,
}
impl PocCall {
    pub fn new(data: &[u8]) -> PocCall {
        let info = *generic::data_const_unsafe::<PocHeader>(
            &data[..size_of::<PocHeader>()]);

        let mut off = size_of::<PocHeader>();
        let fid = data[off..][..info.fid_size].to_vec();
        off += info.fid_size;
        let mem = data[off..][..info.mem_size].to_vec();
        off += info.mem_size;
        let dmp = data[off..][..info.dmp_size].to_vec();

        if dmp.len() != info.dmp_size {
            panic!("[BFL] malformed PocCall/PocHead structure with dmp size mismatch {:X} vs {:X}",
                dmp.len(), info.dmp_size);
        }

        PocCall {
            info: info,
            fid: fid,
            mem: mem,
            dmp: dmp,
        }
    }
    pub fn dump_call(call: &Call, fd: &Fd, uid: u64) -> Vec<u8> {
        let mut call_data = vec![];
        let mem = call.dump_mem();
        let dmp = call.dump_args();
        let total_len = size_of::<PocHeader>() + fd.data().len() + mem.len() + dmp.len(); 
        unsafe {
            call_data.extend(generic::any_as_u8_slice(&total_len));
            call_data.extend(generic::any_as_u8_slice(&call.id()));
            call_data.extend(generic::any_as_u8_slice(&uid));
            call_data.extend(generic::any_as_u8_slice(&fd.data().len()));
            call_data.extend(generic::any_as_u8_slice(&mem.len()));
            call_data.extend(generic::any_as_u8_slice(&dmp.len()));
        }
        call_data.extend(fd.data());
        call_data.extend(mem);
        call_data.extend(dmp);
        call_data
    }
}
