use info::PocDataHeader;
use poc::PocData;


pub fn do_bananized_crossover(
    poc_a: &mut [u8], poc_b: &mut [u8], cross_count: usize,
    ) -> Vec<u8>
{
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

    for i in 0..split_at {
        poc_o.append(poc_a.call(i), poc_a.desc(i).kin);
    }
    for i in cross_at..(cross_at+cross_count) {
        poc_o.append(poc_b.call(i), poc_b.desc(i).kin);
    }
    for i in split_at..poc_a.header().calls_count {
        poc_o.append(poc_a.call(i), poc_a.desc(i).kin);
    }

    poc_o.craft_poc()
}
