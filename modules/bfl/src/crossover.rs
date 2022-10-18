use std::collections::{BTreeMap, BTreeSet};
use rand::{thread_rng, Rng, prelude::IteratorRandom, distributions::{Distribution, WeightedIndex}};

use info::{PocCallHeader, PocDataHeader};
use poc::PocData;

struct Xid {
    pid: u64,
    uid: u64,
    sid: u64,
    ind: u64,
}

impl Xid {
    fn new(pid: u64, uid: u64, sid: u64, ind: u64) -> Self {
        Xid {
            pid: pid,
            uid: uid,
            sid: sid,
            ind: ind,
        }
    }
}

fn find_target(pid: u64, xids: &Vec<Xid>, call: &PocCallHeader) -> Result<Xid, ()> {
    let mut rng = thread_rng();
    if let Some(xid) = xids
        .iter()
        //.filter(|xid| 0 == xid.pid && xid.sid == call.sid)
        .filter(|xid| xid.sid == call.sid)
        .choose(&mut rng)
    {
        return Ok(Xid::new(pid, call.uid, call.sid, xid.ind));
    }
    // this should be by default, SMBC is just for testing
    // - we dont want dup only objects!!
    //    return Err(())

    // seems it is DUP by default, maybe force to disable this option in bananafzz
    Ok(Xid::new(pid, call.uid, call.sid, 1 + xids.len() as u64))
    //Err(())//"No object to related to found!")
}

fn resolve_xid(pid: u64, xids: &mut Vec<Xid>, call: &PocCallHeader) -> Result<(), ()> {
    if 0 == call.level {
        // ctor
        xids.push(Xid::new(pid, call.uid, call.sid, 1 + xids.len() as u64))
    }
    // what about dtor ?
    else if let Some(_) = xids
        .iter()
        .rev()
        .find(|xid| pid == xid.pid && call.uid == xid.uid && call.sid == xid.sid)
    {
        return Ok(());
    } else {
        xids.push(find_target(pid, xids, call)?)
    }
    Ok(())
}

fn adjust_sid(pid: u64, xids: &mut Vec<Xid>, call: &[u8]) -> Result<Vec<u8>, ()> {
    let mut call_vec = call.to_vec();
    let call = generic::data_mut_unsafe::<PocCallHeader>(&mut call_vec);

    resolve_xid(pid, xids, call)?;

    call.uid = xids
        .iter()
        .rev()
        .filter(|xid| xid.uid == call.uid && xid.sid == call.sid)
        .nth(0)
        .unwrap()
        .ind;

    Ok(call_vec)
}

pub fn do_bananized_crossover(poc_a: &[u8], poc_b: &[u8], cross_count: usize) -> Vec<u8> {
    //check loaded spliced memory, if splice is wanted, else return

    let magic = generic::data_const_unsafe::<PocDataHeader>(poc_a).magic;

    let poc_a = unsafe { PocData::new(magic, std::mem::transmute(poc_a.as_ptr())) };
    if poc_a.header().magic != magic {
        panic!("[BFL] splice-A no good magic {:X}", poc_a.header().magic)
    }

    let poc_b = unsafe { PocData::new(magic, std::mem::transmute(poc_b.as_ptr())) };
    if poc_b.header().magic != magic {
        panic!("[BFL] splice-B no good magic {:X}", poc_b.header().magic)
    }

    let mut repro_a = BTreeMap::new();
    let mut repro_b = BTreeMap::new();
    let mut indices = BTreeMap::new();

    repro_a.insert(true, BTreeMap::new());
    repro_a.insert(false, BTreeMap::new());
    repro_b.insert(true, BTreeMap::new());
    repro_b.insert(false, BTreeMap::new());
    indices.insert(true, BTreeMap::new());
    indices.insert(false, BTreeMap::new());

    for i in 0..poc_a.header().calls_count {
        let call = generic::data_const_unsafe::<PocCallHeader>(&poc_a.call(i));

        let ctor = &(0 == call.level);
        let key = if !ctor {
            call.uid
        } else { call.sid };

        if !repro_a[ctor].contains_key(&key) {
            repro_a
                .get_mut(ctor).unwrap()
                .insert(key, 0);
        }
        if !repro_b[ctor].contains_key(&key) {
            repro_b
                .get_mut(ctor).unwrap()
                .insert(key, 0);
            indices
                .get_mut(ctor).unwrap()
                .insert(key, vec![]);
        }
        *repro_a
            .get_mut(&ctor).unwrap()
            .get_mut(&key).unwrap() += 1;
    }

    for i in 0..poc_b.header().calls_count {
        let call = generic::data_const_unsafe::<PocCallHeader>(&poc_b.call(i));

        let ctor = &(0 == call.level);
        let key = if !ctor {
            call.uid
        } else { call.sid };

        if !repro_b[ctor].contains_key(&key) {
            repro_b
                .get_mut(ctor).unwrap()
                .insert(key, 0);
            indices
                .get_mut(ctor).unwrap()
                .insert(key, vec![]);
        }
        *repro_b
            .get_mut(&ctor).unwrap()
            .get_mut(&key).unwrap() += 1;
        indices
            .get_mut(&ctor).unwrap()
            .get_mut(&key).unwrap()
            .push(i);
    }

    let mut xids = vec![];
    let mut seed = rand::thread_rng();
    let mut skip = BTreeSet::new();
    let mut poc_o = PocData::new(magic, 0);
    for i in 0..poc_a.header().calls_count {
        let call = generic::data_const_unsafe::<PocCallHeader>(&poc_a.call(i));

        let ctor = &(0 == call.level);
        let key = &(if !ctor {
            call.uid
        } else { call.sid });

        *repro_a
            .get_mut(ctor).unwrap()
            .get_mut(key).unwrap() -= 1;

        if repro_a[ctor][key] >= repro_b[ctor][key] 
            && seed.gen_bool(0.5) 
        {
            if *ctor {
                skip.insert(call.uid);
            } else { continue } // skip this one ( gen B info = null )
        } // ok keep ( gen A info need prevails )

        if skip.contains(&call.uid) {
            continue
        }

        if 0 == repro_b[ctor][key] || seed.gen_bool(0.5) { // gene A
            let call = adjust_sid(0, &mut xids, poc_a.call(i)).unwrap();
            poc_o.append(&call, poc_a.desc(i).kin);
        } else { // gene B
            let skip = if repro_a[ctor][key] + 1 < repro_b[ctor][key] {
                repro_b[ctor][key] - (repro_a[ctor][key] + 1) - 1
            } else { 0 };

            let skip = WeightedIndex::new(
                (1..=1 + skip).rev()
            ).unwrap().sample(&mut seed);

            *repro_b
                .get_mut(ctor).unwrap()
                .get_mut(key).unwrap() -= (skip + 1);

            let ind = indices[ctor][key].len() - 1 - repro_b[ctor][key];
            let ind = indices[ctor][key][ind];

            let call = adjust_sid(0, &mut xids, poc_b.call(ind)).unwrap();
            poc_o.append(&call, poc_b.desc(ind).kin);
        }
    }
    poc_o.craft_poc()
}

pub fn trim_poc(poc: &[u8], calls_count: usize) -> Vec<u8> {
    let cc = generic::data_const_unsafe::<PocDataHeader>(&poc).calls_count;
    //assert!(cc + 1 >= calls_count);
    let calls_count = if cc < calls_count {
        cc // we stoped fuzzing queue, but plugin after us register that call
    } else { calls_count };

    let magic = generic::data_const_unsafe::<PocDataHeader>(poc).magic;
    let poc = unsafe { PocData::new(magic, std::mem::transmute(poc.as_ptr())) };

    let mut poc_o = PocData::new(magic, 0);

    let mut xids = vec![];
    for i in 0..calls_count {
        if let Ok(call) = adjust_sid(0, &mut xids, poc.call(i)) {
            poc_o.append(&call, poc.desc(i).kin);
        }
    }

    poc_o.craft_poc()
}

pub fn skip(poc: &[u8], index: usize) -> Vec<u8> {
    let cc = generic::data_const_unsafe::<PocDataHeader>(&poc).calls_count;

    let magic = generic::data_const_unsafe::<PocDataHeader>(poc).magic;
    let poc = unsafe { PocData::new(magic, std::mem::transmute(poc.as_ptr())) };

    let mut poc_o = PocData::new(magic, 0);

    let mut xids = vec![];
    for i in (0..cc)
        .filter(|&i| i != index)
    {
        if let Ok(call) = adjust_sid(0, &mut xids, poc.call(i)) {
            poc_o.append(&call, poc.desc(i).kin);
        }
    }

    poc_o.craft_poc()
}
