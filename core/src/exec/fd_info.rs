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
    kin: usize,
}
impl CallInfo {
    pub fn new(success: bool, extra_info: &[u8], kin: usize) -> CallInfo {
        CallInfo {
            success: success,
            extra_info: extra_info.to_vec(),
            kin: kin,
        }
    }
    pub fn success(&self) -> bool { self.success }
    pub fn negate(&mut self) { self.success = !self.success }
    pub fn extra_info(&self) -> &[u8] { &self.extra_info }

    pub fn kin(&self) -> usize { self.kin }

    pub fn fail(kin: usize) -> CallInfo {
        CallInfo::new(false, &[], kin)
    }
    pub fn succ(kin: usize) -> CallInfo {
        CallInfo::new(true, &[], kin)
    }

    pub fn infofromfd(fd: Fd, kin: usize) -> CallInfo {
        CallInfo {
            success: !fd.is_invalid(),
            extra_info: fd.data().to_vec(),
            kin: kin,
        }
   }
}
