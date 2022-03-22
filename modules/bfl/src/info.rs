#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct BananizedFuzzyLoopConfig {
    pub magic: usize,

    pub debug: bool,

    pub shmem: usize,
    pub pocmem: usize,

    pub max_inserts: usize,
    pub max_allowed_wait_count_per_call: usize,
    pub is_strict_repro: bool,

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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PocCallHeader {
    pub len: usize,
    pub cid: u64,
    pub uid: u64,
    pub sid: u64,
    pub level: usize,
    pub fid_size: usize,
    pub mem_size: usize,
    pub dmp_size: usize,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PocDataHeader {
    pub magic: usize,
    pub insert_ind: usize,
    pub split_at: usize,
    pub split_cnt: usize,
    pub total_size: usize,
    pub desc_size: usize,
    pub calls_count: usize,
}
