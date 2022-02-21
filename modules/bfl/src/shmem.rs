use info::PocDataHeader;

#[derive(Clone)]
pub struct ShmemData {
    data: *mut u8,
    size: usize,

    zero: [u8; std::mem::size_of::<PocDataHeader>()],
}
impl ShmemData {
    pub fn new_empty(magic: usize) -> ShmemData {
        unsafe {
            let mut shmem = ShmemData {
                data : std::mem::transmute(0usize),
                size : std::mem::size_of::<PocDataHeader>(),
                zero : [0u8; std::mem::size_of::<PocDataHeader>()],
            };
            shmem.data = std::mem::transmute(shmem.zero.as_ptr());
            shmem.head().magic = magic;
            shmem.head().total_size = shmem.size;
            shmem
        }
    }
    pub unsafe fn new(magic: usize, addr: usize) -> ShmemData {
        let poc : &PocDataHeader = std::mem::transmute(addr);
        if magic != poc.magic {
            panic!("[BFL] shared invalid poc, magic does not match <{:X} vs {:X}>",
                magic, poc.magic)
        }
        ShmemData {
            data : std::mem::transmute(addr),
            size: poc.total_size,
            zero : [0u8; std::mem::size_of::<PocDataHeader>()],
        }
    }
    pub fn data<'a>(&self) -> &'a mut [u8] { 
        unsafe { std::slice::from_raw_parts_mut(self.data, self.size) } 
    }
    pub fn head<'a>(&self) -> &'a mut PocDataHeader { 
        unsafe { std::mem::transmute(self.data) } 
    }
}
