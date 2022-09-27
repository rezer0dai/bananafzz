extern crate log;
use self::log::{debug, error, info, trace, warn};

use std::mem::size_of;

pub use info::PocCallHeader;
pub use info::{PocCallDescription, PocDataHeader};
pub use shmem::ShmemData;

use crossover::do_bananized_crossover;

pub static mut REPROED: bool = false;
pub static mut POCDROP: bool = false;
pub static mut INCOMPLETE: bool = false;

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
    runtime: Vec<(usize, u64, Vec<u8>)>,
    pub added: usize,
    block: Vec<usize>,

    do_gen: bool,
    shared: bool,
    //    calls2: Vec<Vec<u8>>,
    //    descs2: Vec<PocCallDescription>,
}
impl PocData {
    pub fn new(magic: usize, addr: usize) -> PocData {
        let shmem = if 0 != addr {
            unsafe { ShmemData::new(magic, addr) }
        } else {
            ShmemData::new_empty(magic)
        };
        let info = shmem.head().clone();

        let mut poc = if !0 != info.split_at {
            let poc = do_bananized_crossover(
                &mut shmem.data()[..info.total_size],
                &mut shmem.data()[info.total_size..],
                info.split_cnt,
            );

            assert!(0 != poc.len());
            assert!(!0 == generic::data_const_unsafe::<PocDataHeader>(&poc).split_at);

            shmem.data()[..poc.len()].clone_from_slice(&poc);

            shmem.head().insert_ind = info.insert_ind;
            let mut poc = PocData::new(magic, addr);
            poc.do_gen = true;

            poc
        } else {
            shmem.head().insert_ind = !0; // do this by default, if we dont repro case, we always ensure this, so AFL will not add to queue broken input

            let mut poc = PocData {
                magic: magic,

                shmem: shmem.clone(),
                info: info,
                inserted: !0 == info.insert_ind, //means we dont want to append/insert nothing, just repro

                calls: vec![],
                descs: vec![],
                runtime: vec![],
                added: 0,
                block: vec![],
                do_gen: !0 != info.insert_ind,
                shared: false,
                //                calls2: vec![],
                //                descs2: vec![],
            };
            poc.parse_calls();
            poc.parse_descs();
            poc
        };
/*
        if poc.do_gen {
            info!(
                "NEW POC : {:?}",
                (poc.do_gen, poc.inserted, poc.info.insert_ind)
            );

            for i in 0..poc.info.calls_count {
                let desc = poc.desc_data(i);
                info!("? [{i}] Call : {:?}", desc);
                let head = generic::data_const_unsafe::<PocCallHeader>(&shmem.data()[desc.offset..]);
                info!("\n\t?> Header : {:?}", head);
            }
        }
*/
        poc
    }

    pub fn do_gen(&self) -> bool {
        self.do_gen
    }

    pub fn is_generator(&self) -> bool {
        !0 != self.info.split_at || !0 != self.info.insert_ind
    }

    //is expected be called only from do_bananized_crossover
    pub fn append(&mut self, call: &[u8], kin: usize) {
        self.inserted = false;
        self.push(0, call, kin); //internally self.added += 1, which will be the place 0 + self.added

        let poc: &PocCallHeader = generic::data_const_unsafe(call);
        self.runtime(
            poc.level,
            poc.uid,
            &call[std::mem::size_of::<PocCallHeader>()..][..poc.fid_size],
        );

        assert!(self.calls.len() == self.runtime.len());
        self.inserted = true;
    }
    pub fn push(&mut self, ind: usize, call: &[u8], kin: usize) {
        assert!(call.len() > 0);

        if ind + self.added > self.calls.len() {
            panic!(
                "[BFL] how this could happen -> {:?} -> {:?}",
                (ind, self.added, self.calls.len()),
                (self.info.calls_count, self.info.insert_ind)
            );
        }
        let ind = ind + self.added; // - self.block.len();
/*
        if self.inserted {
            // && ind != self.info.insert_ind {
            return; // we already inserted one as we are in INSERT MODE of AFL
        }
*/
        self.added += 1;

        self.descs.insert(
            ind,
            PocCallDescription {
                offset: match ind {
                    pos if self.calls.len() == pos => self.info.total_size,
                    _ => self.descs[ind].offset,
                },
                size: call.len(),
                kin: kin,
            },
        );
        self.calls.insert(ind, call.to_vec());

        for i in 0..self.descs.len() {
            self.descs[i].offset += size_of::<PocCallDescription>();
            if i < ind + 1 {
                continue;
            }
            self.descs[i].offset += call.len();
        }

        // total_size is not used for orig poc so we can update now
        // + basically call.len() will be originally hard to query later on
        self.info.total_size += call.len() + size_of::<PocCallDescription>();
    }
    fn drop_blocked(&mut self) {
        for ind in self.block.drain(..).enumerate().map(|(i, ind)| ind - i) {
            if self.descs.len() == ind {
                break
            }
            self.descs.remove(ind);
            let call = self.calls.remove(ind);
            //ok we need to adjust offset table too
            for i in 0..self.descs.len() {
                self.descs[i].offset -= size_of::<PocCallDescription>();
                if i < ind {
                    continue;
                }
                self.descs[i].offset -= call.len();
            }
            trace!(
                "[{ind}] : total size : {:?} - {:?}",
                self.info.total_size,
                (size_of::<PocCallDescription>(), call.len())
            );
            self.info.total_size -= size_of::<PocCallDescription>() + call.len();
        }
    }
    pub fn craft_poc(&mut self) -> Vec<u8> {
        // by default we want mark all pocs from banana as repro-only, AFL choosing mode afterwards
        if self.descs.len() != self.calls.len() {
            panic!("NOT MADE EQUAL");
        }
        if 0 == self.runtime.len() {
            error!("not a signle call did goes trough");
            return vec![];
        }

        println!(
            "CRAFT POC => {:?} x {:?}",
            self.calls.len(),
            self.runtime.len(),
        );

        self.drop_blocked(); // drop blocked in between
        while self.calls.len() != self.runtime.len() + self.block.len() {
            self.block.push(self.runtime.len() + self.block.len());
        } // trim if runtime < calls, we gots blocked somewhere ( limiter, too much inserted in the middle )
        self.drop_blocked(); // drop blocked after trim

        self.info.calls_count = self.calls.len();
        self.info.desc_size = self.info.calls_count * size_of::<PocCallDescription>();
        self.info.insert_ind = !0;
        self.info.split_at = !0;
        self.info.split_cnt = 0;

        assert!(0 != self.info.calls_count);

        assert!(
            self.calls.len() == self.runtime.len(),
            "#calls x #runtime => {:?}",
            (self.calls.len(), self.runtime.len())
        );

        let mut data = vec![];
        data.extend_from_slice(unsafe { generic::any_as_u8_slice(&self.info) });

        data.extend(
            self.descs
                .iter()
                .enumerate()
                .map(|(_, desc)| unsafe { generic::any_as_u8_slice(desc) })
                .flat_map(move |data| data.to_vec())
                .collect::<Vec<u8>>(),
        );

        data.extend(
            self.calls
                .iter_mut()
                .zip(self.runtime.iter())
                .flat_map(move |(call, &(level, uid, ref fid))| {
                    call[std::mem::size_of::<PocCallHeader>()..][..fid.len()]
                        .clone_from_slice(&fid);
                    let mut c = generic::data_mut_unsafe::<PocCallHeader>(call);
                    c.level = level;
                    c.uid = uid;
                    call.to_vec()
                })
                .collect::<Vec<u8>>(),
        );
        /*
        unsafe {//lets sanitize
            println!("\n\n DOUBLE CHECK!!");
        let poc = PocData::new(66, std::mem::transmute(data.as_ptr()));
            println!("\n\n CHECKED!!");

        //lets do sanity checks!!
        if self.info.total_size != data.len() { return vec![] }
        for i in 0..poc.info.calls_count {
            let desc = poc.desc(i);
            if desc.offset > poc.shmem.data().len() {
                loop { println!("BAD POC#1") }
                return vec![]
            }
            let head = generic::data_const_unsafe::<PocCallHeader>(&poc.shmem.data()[desc.offset..]);
            if head.len != desc.size {
                loop { println!("BAD POC#2") }
                return vec![]
            }
        }

        }
        */
        data
    }

    fn upload_poc(&mut self, addr: usize) -> bool {
        /*
        if !self.do_gen {
            return true;
        }
        */
        let data = self.craft_poc();
        if 0 == data.len() {
            warn!("POC0");
            return false;
        }
        unsafe { POCDROP = true } //temporary
        unsafe { generic::c_memcpy(addr, &data) };
        true
    }
    pub fn is_last_call(&self, ind: usize) -> bool {
        return ind == self.info.calls_count;
    }
    //write to shared memory - pocout
    pub fn share(&mut self, addr: usize) -> bool {
        let inserted = self.inserted;

        self.inserted = true;
        if !inserted {//&& self.info.calls_count != self.info.insert_ind {
            /***************/
            /* INSERT MODE */
            /***************/
            return false; // insert in between and continue until end of repro
        }
        if self.shared {
            return true
        }
        self.info.magic = self.magic;
        if self.upload_poc(addr) {
            info!("UPLOADED [{:?} (/{:?})] # {:?}", self.runtime.len(), self.info.calls_count, if self.do_gen { "generative" } else { "repro" });
            self.shared = true;
            unsafe { REPROED = true } //temporary
        } else {
            error!("UPLOAD FAILED");
        }
        true
    }
    pub fn skip(&mut self, ind: usize) {
trace!("SKIPED ONE");
        self.block.push(ind + self.added);
    }
    pub fn add_one(&mut self, new_ind: usize) -> bool {
trace!("ADDED ONE {new_ind} vs {:?}", self.info.calls_count);
        self.inserted = false;
        self.info.insert_ind = new_ind;
        false
    }
    pub fn runtime(&mut self, level: usize, uid: u64, fid: &[u8]) {
        self.runtime.push((level, uid, fid.to_vec()))
    }
    pub fn max_ind(&self) -> usize {
        if !self.inserted {
            return self.info.insert_ind;
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

    pub fn load(&mut self, ind: usize) -> &[u8] {
        //if self.calls2.len() > 0 { return &self.calls2[ind] }
        let call = self.desc_data(ind).clone();
        while call.offset > self.shmem.data().len() {
            panic!("--> CALL {call:?}");
        }
        &self.shmem.data()[call.offset..][..call.size]
    }
    pub fn desc_data(&mut self, ind: usize) -> &mut PocCallDescription {
        //if self.descs2.len() > 0 { return &mut self.descs2[ind] }
        let desc = &mut self.shmem.data()
            [size_of::<PocDataHeader>() + ind * size_of::<PocCallDescription>()..]
            [..size_of::<PocCallDescription>()];

        generic::data_mut_unsafe(desc)
    }

    fn parse_calls(&mut self) {
        self.calls = (0..self.info.calls_count)
            .map(|ind| self.load(ind).to_vec())
            .collect::<Vec<Vec<u8>>>();
        //        self.calls2 = self.calls.clone();
    }
    fn parse_descs(&mut self) {
        self.descs = (0..self.info.calls_count)
            .map(|ind| *self.desc_data(ind))
            .collect::<Vec<PocCallDescription>>();
        //        self.descs2 = self.descs2.clone();
    }
}
