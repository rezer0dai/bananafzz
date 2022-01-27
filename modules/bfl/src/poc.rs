use std::mem::size_of;

pub enum ShmemId {
    PocIn = 1,
    PocOut = 2,
    SpliceA = 3,
    SpliceB = 4,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct BananizedFuzzyLoopConfig {
    pub magic: usize,

    shmem_pocin: i32,
    shmem_pocout: i32,
    shmem_splicea: i32,
    shmem_spliceb: i32,

    pub warmup_cnt: usize,
    pub ctor_min_ratio: usize,
    pub ctor_max_ratio: usize,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PocCallDescription {
    pub offset: usize,
    pub size: usize,
    pub kin: usize,
}

pub use shmem::{ShmemData, PocDataHeader};

pub struct PocData {
    cfg: BananizedFuzzyLoopConfig,

    shmem: ShmemData,
    info: PocDataHeader,
    inserted: bool,

    calls: Vec<Vec<u8>>,
    descs: Vec<PocCallDescription>,
}
impl PocData {
    pub fn new(config: &BananizedFuzzyLoopConfig, shmem_id: ShmemId) -> PocData {
        let shmem = PocData::load_shmem(config, shmem_id);
        let info = *generic::data_const_unsafe::<PocDataHeader>(shmem.data());
        let mut poc = PocData {
            cfg: *config,

            shmem: shmem,
            info: info,
            inserted: 0 == info.insert_ind,//means we dont want to append/insert nothing, just repro

            calls: vec![vec![]],
            descs: vec![],
        };
        poc.parse_calls();
        poc.parse_descs();
        poc
    }

    pub fn append(&mut self, call: &[u8], kin: usize) {
        self.push(self.calls.len(), call, kin)
    }
    pub fn push(&mut self, ind: usize, call: &[u8], kin: usize) {
        assert!(self.info.calls_count == self.calls.len());

        if self.inserted && self.info.calls_count != self.info.insert_ind { 
            return // we already inserted one as we are in INSERT MODE of AFL
        }

        self.calls.insert(ind, call.to_vec());
        self.descs.insert(ind, PocCallDescription {
            offset : if 0 == ind { 0 } else { 
                self.descs[ind-1].offset + self.descs[ind-1].size },
            size : call.len(),
            kin : kin,
        });

        self.info.total_size += call.len() + size_of::<PocCallDescription>();
        self.info.desc_size += size_of::<PocCallDescription>();
        self.info.calls_count += 1;
    }
    pub fn discard(&mut self) {
        self.info.magic = 0;
        self.upload_poc();
    }
    fn upload_poc(&mut self) {
        let mut data = vec![];

// by default we want mark all pocs from banana as repro-only, AFL choosing mode afterwards
        self.info.insert_ind = 0; 

        data.extend_from_slice(
            unsafe { generic::any_as_u8_slice(&self.info) });
        data.extend(
            self.descs
                .iter()
                .map(|desc| unsafe { generic::any_as_u8_slice(desc) } )
                .flat_map(move |data| data.to_vec())
                .collect::<Vec<u8>>());
        data.extend(
            self.calls
                .iter()
                .flat_map(move |call| call.to_vec())
                .collect::<Vec<u8>>());

        self.shmem.upload(&data);
    }
//write to shared memory - pocout
    pub fn share(&mut self) -> bool {
        let inserted = self.inserted;

        self.inserted = true;
        if !inserted && self.info.calls_count != self.info.insert_ind { 
            /***************/
            /* INSERT MODE */
            /***************/
            return true// insert in between and continue until end of repro
        }
        
        self.info.magic = self.cfg.magic;
        self.upload_poc();
        std::process::exit(0)
    }
    pub fn max_ind(&self) -> usize {
        if !self.inserted {
            return self.info.insert_ind
        }
        self.info.calls_count
    }
    pub fn broken_poc() {
        std::process::exit(0)
    }

    pub fn call(&self, ind: usize) -> &[u8] {
        &self.calls[ind]
    }
    pub fn desc(&self, ind: usize) -> &PocCallDescription {
        &self.descs[ind]
    }
    pub fn header(&self) -> &PocDataHeader {
        &self.info
    }

    pub fn load(&self, ind: usize) -> &[u8] {
        if 0 != self.calls.len() {
            panic!("[BFL] internal Banana plugin fail - using PocData::load function after shmem.data is possibly detached from PocData.data")
        }
        let call = self.desc_data(ind);
        &self.shmem.data()[
            size_of::<PocDataHeader>() + 
            self.info.desc_size + 
            call.offset..
        ][..call.size]
    }
    fn desc_data(&self, ind: usize) -> &PocCallDescription {
        if 0 != self.descs.len() {
            panic!("[BFL] internal Banana plugin fail - using PocData::desc_data function after shmem.data is possibly detached from PocData.data")
        }
        generic::data_const_unsafe(
            &self.shmem.data()[
                size_of::<PocDataHeader>() + 
                ind * size_of::<PocCallDescription>()..
            ][..size_of::<PocCallDescription>()])
    }

    fn load_shmem(config: &BananizedFuzzyLoopConfig, shmem_id: ShmemId) -> ShmemData {
        match shmem_id {
            ShmemId::PocOut => {
                let data = vec![0u8; size_of::<PocDataHeader>()]; 
                let mut poc = *generic::data_const_unsafe::<PocDataHeader>(&data);
                poc.magic = config.magic;
                poc.insert_ind = 0;//do repro only by default
                poc.total_size = size_of::<PocDataHeader>();
                ShmemData::new_with_data(data, config.shmem_pocout)
            },
            ShmemId::PocIn => {
                ShmemData::new(config.magic, config.shmem_pocin)
            },
            ShmemId::SpliceA => {
                ShmemData::new(config.magic, config.shmem_splicea)
            },
            ShmemId::SpliceB => {
                ShmemData::new(config.magic, config.shmem_spliceb)
            },
        }
    }
    fn parse_calls(&mut self) {
        self.calls = (0..self.info.calls_count)
            .map(|ind| self.load(ind).to_vec())
            .collect::<Vec<Vec<u8>>>();
    }
    fn parse_descs(&mut self) {
        self.descs = (0..self.info.calls_count)
            .map(|ind| *self.desc_data(ind))
            .collect::<Vec<PocCallDescription>>();
    }
}
