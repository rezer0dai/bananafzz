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
            shmem.head().insert_ind = !0;
            shmem.head().split_at = !0;
            shmem
        }
    }
    pub unsafe fn new(magic: usize, addr: usize) -> ShmemData {
        let poc : &mut PocDataHeader = std::mem::transmute(addr);
        if magic != poc.magic {
            println!("[BFL] shared invalid poc, magic does not match <{:X} vs {:X}>",
                magic, poc.magic)
        }
        let size = if !0 != poc.split_at {
            let cross : &PocDataHeader = std::mem::transmute(addr + poc.total_size);

            let mut size = poc.total_size;

            if magic != cross.magic || !0 == cross.split_at
                || poc.split_cnt + cross.split_at > cross.calls_count {
                poc.split_at = !0;
            } else { 
                size += cross.total_size };

            size
        } else {//if magic == poc.magic { 
            poc.total_size 
        };// else { 0 };

        ShmemData {
            data : std::mem::transmute(addr),
            size: size,
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
