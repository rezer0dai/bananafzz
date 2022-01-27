#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PocDataHeader {
    pub magic: usize,
    pub insert_ind: usize,
    pub total_size: usize,
    pub desc_size: usize,
    pub calls_count: usize,
}

pub struct ShmemData {
    addr: usize,
    data: Vec<u8>
}
impl ShmemData {
    pub fn new(magic: usize, tag: i32) -> ShmemData {
        unsafe {
            let addr: usize = std::mem::transmute(
                libc::shmat(tag, std::mem::transmute(0usize), 0)); if 0 == addr {
                panic!("[BFL] shared invalid address")
            }
            let poc : &PocDataHeader = std::mem::transmute(addr);
            if magic != poc.magic {
                panic!("[BFL] shared invalid poc, magic does not match <{:X} vs {:X}>",
                    magic, poc.magic)
            }
            let mut data = vec![0u8; poc.total_size];
            generic::c_memload(addr, &mut data);

            ShmemData {
                addr : addr,
                data : data,
            }
        }
    }
    pub fn new_with_data(data: Vec<u8>, tag: i32) -> ShmemData {
        unsafe {
            let addr: usize = std::mem::transmute(
                libc::shmat(tag, std::mem::transmute(0usize), 0));
            if 0 == addr {
                panic!("[BFL] shared invalid address")
            }
            ShmemData {
                addr : addr,
                data : data,
            }
        }
    }
    pub fn data(&self) -> &[u8] { &self.data }

    pub fn upload(&mut self, data: &[u8]) {
        unsafe {
            generic::c_memcpy(self.addr, data)
        }
    }
}
impl Drop for ShmemData {
    fn drop(&mut self) {
        unsafe {
            libc::shmdt(std::mem::transmute(self.addr));
        }
    }
}
