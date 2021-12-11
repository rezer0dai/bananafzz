#[derive(Clone, Debug)]
pub struct Fd {
    data: Vec<u8>
}

impl Fd {
    pub fn new(data: &[u8]) -> Fd {
        Fd { data : data.to_vec() }
    }

    pub fn data(&self) -> &[u8] { &self.data }
    pub fn equals(&self, fd: &Fd) -> bool { self.data() == fd.data() }

    pub fn init(&mut self, fd: &Fd) {
        self.data = fd.data().to_vec();
    }

    pub fn empty() -> Fd {
        Fd { data : Vec::new() }
    }
    pub fn dummy(size: usize) -> Fd {
        Fd { data : vec![0x00u8; size] }
    }
    pub fn invalid(size: usize) -> Fd {
        Fd { data : vec![0xFFu8; size] }
    }

    pub fn is_invalid(&self) -> bool {
        if self.data.is_empty() {
            return true
        }
        self.data
            .iter()
            .all(|&b| 0xFF == b) || self.data
                                        .iter()
                                        .all(|&b| 0x00 == b)
    }
}

pub struct CallInfo {
    success: bool,
    extra_info: Vec<u8>,
}
impl CallInfo {
    pub fn new(success: bool, extra_info: &[u8]) -> CallInfo {
        CallInfo {
            success: success,
            extra_info: extra_info.to_vec(),
        }
    }
    pub fn success(&self) -> bool { self.success }
    pub fn negate(&mut self) { self.success = !self.success }
    pub fn extra_info(&self) -> &[u8] { &self.extra_info }

    pub fn fail() -> CallInfo {
        CallInfo::new(false, &[])
    }
    pub fn succ() -> CallInfo {
        CallInfo::new(true, &[])
    }

    pub fn info(data: u8) -> CallInfo {
        CallInfo {
            success: 0 != data && !0 != data,
            extra_info: vec![data, 8],
        }
   }

   pub fn infofromfd(fd: Fd) -> CallInfo {
        CallInfo {
            success: !fd.is_invalid(),
            extra_info: fd.data().to_vec(),
        }
   }
}
