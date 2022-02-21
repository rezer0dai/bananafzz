use std::mem::size_of;

pub use shmem::ShmemData;
pub use info::{PocCallDescription, PocDataHeader};
pub use info::PocCallHeader;

pub enum ShmemId {
    PocIn = 1,
    PocOut = 2,
    SpliceA = 3,
    SpliceB = 4,
}

pub struct PocData {
    magic: usize,

    shmem: ShmemData,
    info: PocDataHeader,
    inserted: bool,

    calls: Vec<Vec<u8>>,
    descs: Vec<PocCallDescription>,
}
impl PocData {
    pub fn new(magic: usize, addr: usize) -> PocData {
        let shmem = if 0 != addr { 
            unsafe { ShmemData::new(magic, addr)}
        } else { ShmemData::new_empty(magic) };
        let info = shmem.head().clone();
        let mut poc = PocData {
            magic: magic,

            shmem: shmem.clone(),
            info: info,
            inserted: !0 == info.insert_ind,//means we dont want to append/insert nothing, just repro

            calls: vec![vec![]],
            descs: vec![],
        };
//println!("[BFL] how is that {:?}", poc.calls.len());
        poc.parse_calls();
        poc.parse_descs();
/*
println!("==> INFO {:?}", poc.info);
for i in 0..poc.info.calls_count {
println!("??");
    let desc = poc.desc_data(i);
println!("? Call : {:?}", desc);
    let head = generic::data_const_unsafe::<PocCallHeader>(&shmem.data()[desc.offset..]);
println!("\n\t?> Header : {:?}", head);
}
*/
        poc
    }

    pub fn append(&mut self, call: &[u8], kin: usize) {
        self.push(self.calls.len(), call, kin)
    }
    pub fn push(&mut self, ind: usize, call: &[u8], kin: usize) {
//        assert!(self.info.calls_count == self.calls.len());

        if self.inserted {// && ind != self.info.insert_ind { 
            return // we already inserted one as we are in INSERT MODE of AFL
        }
//println!("PUSHED {:?}", ind);

//println!("[BFL] CALLS manual insert");
        self.descs.insert(ind, PocCallDescription {
            offset : match ind {
                    0 => size_of::<PocDataHeader>(),
                    pos if self.calls.len() == pos => self.info.total_size,
                    _ => self.descs[ind].offset,
                },
            size : call.len(),
            kin : kin,
        });
        self.calls.insert(ind, call.to_vec());

        for i in 0..self.descs.len() {
            self.descs[i].offset += size_of::<PocCallDescription>();
            if i < ind + 1 {
                continue
            }
//println!("UPDATING : {i}/{ind}/{:?} :: {:?}", self.descs.len(), call.len());
            self.descs[i].offset += call.len();
        }

if 0 == call.len() { panic!("[BFL] CALL LEN == 0") }

        // total_size is not used for orig poc so we can update now
        // + basically call.len() will be originally hard to query later on
        self.info.total_size += call.len() + size_of::<PocCallDescription>();
//        self.info.desc_size += size_of::<PocCallDescription>();
//        self.info.calls_count += 1;
//println!("[BFL] updated total size (A) : {:?}", self.info.total_size);
    }
    pub fn craft_poc(&mut self) -> Vec<u8> {
        let mut data = vec![];

// by default we want mark all pocs from banana as repro-only, AFL choosing mode afterwards
        self.info.calls_count = self.calls.len();
        if 0 == self.info.calls_count {
            return vec![]
        }
        self.info.desc_size = self.calls.len() * size_of::<PocCallDescription>();
        self.info.insert_ind = !0; 

        data.extend_from_slice(
            unsafe { generic::any_as_u8_slice(&self.info) });
        data.extend(
            self.descs
                .iter()
                .map(|desc| unsafe { 
if 0 == desc.size { panic!("[BFL] 0 call desc size") }
                    generic::any_as_u8_slice(desc) } )
                .flat_map(move |data| data.to_vec())
                .collect::<Vec<u8>>());
        data.extend(
            self.calls
                .iter()
                .flat_map(move |call| call.to_vec())
                .collect::<Vec<u8>>());
//        data.hash = hash(data[8..]);
        data
    }

    fn upload_poc(&mut self, addr: usize) {
        if !0 == self.info.insert_ind {
            return
        }
        let data = self.craft_poc();
//println!("[BFL] updated total size (B) : {:?}  [{:?} : {:?}]", self.info.total_size, self.info.desc_size, self.info.calls_count);
//println!("UPLOADING TO {:X}", addr);
        unsafe { generic::c_memcpy(addr, &data) };
    }
//write to shared memory - pocout
    pub fn share(&mut self, addr: usize) -> bool {
        let inserted = self.inserted;

        self.inserted = true;
        if !inserted && self.info.calls_count != self.info.insert_ind { 
//println!("FAILED TO UPDATE {:?}", self.info.insert_ind);
            /***************/
            /* INSERT MODE */
            /***************/
            return false// insert in between and continue until end of repro
        }
        
        self.info.magic = self.magic;
        self.upload_poc(addr);
//        std::process::exit(0)
        true
    }
    pub fn max_ind(&self) -> usize {
        if !self.inserted {
            return self.info.insert_ind
        }
        self.info.calls_count
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

    pub fn load(&mut self, ind: usize) -> &mut[u8] {
        let call = self.desc_data(ind).clone();
//println!("[BFL] call : {:?}:<{:?}:{:?}>", ind, call.offset, call.size);
        &mut self.shmem.data()[call.offset..][..call.size]
    }
    pub fn desc_data(&mut self, ind: usize) -> &mut PocCallDescription {
        let desc = &mut self.shmem.data()[
                size_of::<PocDataHeader>() + 
                ind * size_of::<PocCallDescription>()..
            ][..size_of::<PocCallDescription>()];

//println!("[BFL] id : {:X} => {:?} + {:?} || {:?} vs {:?}", ind, self.inserted, self.info.insert_ind, size_of::<PocCallDescription>(), desc.len());

        generic::data_mut_unsafe(desc)
    }

    fn parse_calls(&mut self) {
//println!("[BFL] CALLS parse {:?}", self.calls.len());
        self.calls = (0..self.info.calls_count)
            .map(|ind| self.load(ind).to_vec())
            .collect::<Vec<Vec<u8>>>();
//println!("[BFL] CALLS parsed");
    }
    fn parse_descs(&mut self) {
        self.descs = (0..self.info.calls_count)
            .map(|ind| {
//println!("[BFL] desc {:?} => {:?}", ind, self.desc_data(ind).size);
                *self.desc_data(ind)
            })
            .collect::<Vec<PocCallDescription>>();
    }
}
