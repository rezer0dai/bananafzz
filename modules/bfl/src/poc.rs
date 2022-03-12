use std::mem::size_of;

pub use shmem::ShmemData;
pub use info::{PocCallDescription, PocDataHeader};
pub use info::PocCallHeader;

use crossover::do_bananized_crossover;

pub static mut REPROED: bool = false;
pub static mut POCDROP: bool = false;

pub enum ShmemId {
    PocIn = 1,
    PocOut = 2,
    SpliceA = 3,
    SpliceB = 4,
}

pub struct PocData {
    magic: usize,

    shmem: ShmemData,
    pub info: PocDataHeader,
    inserted: bool,

    calls: Vec<Vec<u8>>,
    descs: Vec<PocCallDescription>,
    runtime: Vec<(usize, u64)>,
    added: usize,
    block: Vec<usize>,

    do_gen: bool,
}
impl PocData {
    pub fn new(magic: usize, addr: usize) -> PocData {
        let mut shmem = if 0 != addr { 
            unsafe { ShmemData::new(magic, addr)}
        } else { ShmemData::new_empty(magic) };
        let mut info = shmem.head().clone();

        let do_gen = !0 != info.split_at || !0 != info.insert_ind;

        let mut poc = if !0 != info.split_at {
            let poc = do_bananized_crossover(
                &mut shmem.data()[..info.total_size], 
                &mut shmem.data()[info.total_size..], 
                info.split_cnt,
                );

            assert!(0 != poc.len());
            assert!(!0 == generic::data_const_unsafe::<PocDataHeader>(&poc).split_at);

            shmem
                .data()[..poc.len()]
                .clone_from_slice(&poc);

            shmem.head().insert_ind = info.insert_ind;
            let mut poc = PocData::new(magic, addr);
            poc.do_gen = do_gen;

            poc
        } else {

            shmem.head().insert_ind = !0; // do this by default, if we dont repro case, we always ensure this, so AFL will not add to queue broken input

            let mut poc = PocData {
                magic: magic,

                shmem: shmem.clone(),
                info: info,
                inserted: !0 == info.insert_ind,//means we dont want to append/insert nothing, just repro

                calls: vec![vec![]],
                descs: vec![],
                runtime: vec![],
                added: 0,
                block: vec![],
                do_gen: !0 != info.insert_ind,
            };
            poc.parse_calls();
            poc.parse_descs();
            poc
        };
        /*

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

    pub fn do_gen(&self) -> bool { self.do_gen }

    pub fn is_generator(&self) -> bool { !0 != self.info.split_at || !0 != self.info.insert_ind }

    //is expected be called only from do_bananized_crossover
    pub fn append(&mut self, call: &[u8], kin: usize) {
        self.push(self.calls.len(), call, kin);

        let call: &PocCallHeader = generic::data_const_unsafe(call);
        self.runtime(call.level, call.uid);
    }
    pub fn push(&mut self, ind: usize, call: &[u8], kin: usize) {
//        assert!(self.info.calls_count == self.calls.len());
        let ind = ind + self.added;

        if self.inserted {// && ind != self.info.insert_ind { 
            return // we already inserted one as we are in INSERT MODE of AFL
        }

        self.descs.insert(ind, PocCallDescription {
            offset : match ind {
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
            self.descs[i].offset += call.len();
        }

        // total_size is not used for orig poc so we can update now
        // + basically call.len() will be originally hard to query later on
        self.info.total_size += call.len() + size_of::<PocCallDescription>();
//        self.info.desc_size += size_of::<PocCallDescription>();
//        self.info.calls_count += 1;
    }
    pub fn craft_poc(&mut self) -> Vec<u8> {
// by default we want mark all pocs from banana as repro-only, AFL choosing mode afterwards
        self.info.calls_count = self.calls.len() - self.block.len();
        if 0 == self.info.calls_count {
            return vec![]
        }
        self.info.desc_size = self.info.calls_count * size_of::<PocCallDescription>();
        self.info.insert_ind = !0; 
        self.info.split_at = !0; 
        self.info.split_cnt = 0; 

        if 0 != self.runtime.len() {
            if self.calls.len() > self.runtime.len() {
                return vec![]//failed to repro!!
            }
        } else { // inserting first call, or crossover (handled by crossover should be -> now at append)
            assert!(1 == self.info.calls_count, "[BFL] zero levels but non zero ({:?}) calls", self.calls.len());
            let call = generic::data_const_unsafe::<PocCallHeader>(&self.calls[0]);
            self.runtime = vec![(call.level, call.uid)];
        }

        let mut data = vec![];
        data.extend_from_slice(
            unsafe { generic::any_as_u8_slice(&self.info) });
        data.extend(
            self.descs
                .iter()
                .enumerate()
                .filter(|(i, _)| !self.block.contains(i))
                .map(|(_, desc)| unsafe { generic::any_as_u8_slice(desc) } )
                .flat_map(move |data| data.to_vec())
                .collect::<Vec<u8>>());

        let block = self.block.clone();
        data.extend(
            self.calls
                .iter_mut()
                .zip(self.runtime.iter())
                .enumerate()
                .filter(|(i, _)| !block.contains(i))
                .flat_map(move |(_, (call, &(level, uid)))| {
                    let mut c = generic::data_mut_unsafe::<PocCallHeader>(call);
                    c.level = level;
                    c.uid = uid;
                    call.to_vec()
                })
                .collect::<Vec<u8>>());
        data
    }

    fn upload_poc(&mut self, addr: usize) -> bool {
        if !0 == self.info.insert_ind {
            return true
        }
        let data = self.craft_poc();
        if 0 == data.len() {
            return false
        }
        unsafe { POCDROP = true }//temporary
        unsafe { generic::c_memcpy(addr, &data) };
        true
    }
    pub fn is_last_call(&self) -> bool {
        if 0 == self.info.calls_count {
            return false
        }//exception for generation of first entry
        self.inserted || self.info.calls_count == self.info.insert_ind
    }
//write to shared memory - pocout
    pub fn share(&mut self, addr: usize) -> bool {
        let inserted = self.inserted;

        self.inserted = true;
        if !inserted && self.info.calls_count != self.info.insert_ind { 
            /***************/
            /* INSERT MODE */
            /***************/
            return false// insert in between and continue until end of repro
        }
        self.info.magic = self.magic;
        if self.upload_poc(addr) {
            unsafe { REPROED = true }//temporary
        }
//        std::process::exit(0)
        true
    }
    pub fn skip(&mut self, ind: usize) {
        self.block.push(ind)
    }
    pub fn add_one(&mut self, new_ind: usize) -> bool {
        self.added += 1;
        self.inserted = false;
        self.info.insert_ind = new_ind;
        false
    }
    pub fn runtime(&mut self, level: usize, uid: u64) {
        self.runtime.push((level, uid))
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
        &mut self.shmem.data()[call.offset..][..call.size]
    }
    pub fn desc_data(&mut self, ind: usize) -> &mut PocCallDescription {
        let desc = &mut self.shmem.data()[
                size_of::<PocDataHeader>() + 
                ind * size_of::<PocCallDescription>()..
            ][..size_of::<PocCallDescription>()];

        generic::data_mut_unsafe(desc)
    }

    fn parse_calls(&mut self) {
        self.calls = (0..self.info.calls_count)
            .map(|ind| self.load(ind).to_vec())
            .collect::<Vec<Vec<u8>>>();
    }
    fn parse_descs(&mut self) {
        self.descs = (0..self.info.calls_count)
            .map(|ind| {
                *self.desc_data(ind)
            })
            .collect::<Vec<PocCallDescription>>();
    }
}
