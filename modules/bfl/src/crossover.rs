use rand::prelude::IteratorRandom;
use rand::thread_rng;

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
        .filter(|xid| 0 == xid.pid && xid.sid == call.sid)
        .choose(&mut rng)
    {
        return Ok(Xid::new(pid, call.uid, call.sid, xid.ind));
    }
    // this should be by default, SMBC is just for testing
    // - we dont want dup only objects!!
    //    return Err(())

    // seems it is DUP by default, maybe force to disable this option in bananafzz
    //Ok(Xid::new(pid, call.uid, call.sid, 1 + xids.len() as u64))
    Err(())//"No object to related to found!")
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

pub fn do_bananized_crossover(poc_a: &mut [u8], poc_b: &mut [u8], cross_count: usize) -> Vec<u8> {
    //check loaded spliced memory, if splice is wanted, else return

    let magic = generic::data_const_unsafe::<PocDataHeader>(poc_a).magic;
    let split_at = generic::data_const_unsafe::<PocDataHeader>(poc_a).split_at;
    assert!(!0 != split_at);
    generic::data_mut_unsafe::<PocDataHeader>(poc_a).split_at = !0;
    let poc_a = unsafe { PocData::new(magic, std::mem::transmute(poc_a.as_ptr())) };
    if poc_a.header().magic != magic {
        panic!("[BFL] splice-A no good magic {:X}", poc_a.header().magic)
    }

    let cross_at = generic::data_const_unsafe::<PocDataHeader>(poc_b).split_at;
    assert!(!0 != cross_at);
    generic::data_mut_unsafe::<PocDataHeader>(poc_b).split_at = !0;
    let poc_b = unsafe { PocData::new(magic, std::mem::transmute(poc_b.as_ptr())) };
    if poc_b.header().magic != magic {
        panic!("[BFL] splice-B no good magic {:X}", poc_b.header().magic)
    }

    let mut poc_o = PocData::new(magic, 0);

    let mut xids = vec![];
    for i in 0..split_at {
        if let Ok(call) = adjust_sid(0, &mut xids, poc_a.call(i)) {
            poc_o.append(&call, poc_a.desc(i).kin);
        }
    }
    for i in cross_at..(cross_at + cross_count) {
        if let Ok(call) = adjust_sid(1, &mut xids, poc_b.call(i)) {
            poc_o.append(&call, poc_b.desc(i).kin);
        }
    }
    for i in split_at..poc_a.header().calls_count {
        if let Ok(call) = adjust_sid(0, &mut xids, poc_a.call(i)) {
            poc_o.append(&call, poc_a.desc(i).kin);
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
