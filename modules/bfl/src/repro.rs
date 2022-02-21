use std::mem::size_of;

use core::exec::call::Call;
use core::exec::fd_info::Fd;

pub use info::PocCallHeader;

pub struct PocCall {
    pub info: PocCallHeader,
    pub fid: Vec<u8>,
    pub mem: Vec<u8>,
    pub dmp: Vec<u8>,
}
impl PocCall {
    pub fn new(data: &[u8]) -> PocCall {
        let info = *generic::data_const_unsafe::<PocCallHeader>(&data);

        let mut off = size_of::<PocCallHeader>();
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
        let mut call_data = vec![0u8; size_of::<PocCallHeader>()];
        let mem = call.dump_mem();
        let dmp = call.dump_args();

        unsafe {
            let mut head = generic::data_mut_unsafe::<PocCallHeader>(&mut call_data);
            head.cid = call.id().into();
            head.uid = uid;
            head.fid_size = fd.data().len();
            head.mem_size = mem.len();
            head.dmp_size = dmp.len();
        }
        call_data.extend(fd.data());
        call_data.extend(mem);
        call_data.extend(dmp);

        let total_size = call_data.len();
        unsafe {
            let mut head = generic::data_mut_unsafe::<PocCallHeader>(&mut call_data);
            head.len = total_size;
        }
        call_data
    }
}
