extern crate toml;
extern crate generic;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FuzzyConfig {
    pub version: String,
    pub noisy: bool,

    pub dead_call: f64,
    pub afl_fix_ratio: f64,

    pub unicorn_kin_limit: usize,
    pub ratio: usize,
    pub max_racers_count: usize,
    pub max_queue_size: usize,

    pub new_limit: usize,
    pub dup_limit: usize,
    pub rep_limit: usize,

// fuzzer specifics
    pub active_time: u64,
    pub push_sleep: u64,
    pub after_creation_sleep: u64,
    pub failing_delay_limit: u64,
    pub generate_failing_delay: usize,
// modules related sync
    pub n_cores: u64,
    pub wait_max: u64,

    pub push_count: u64,

    #[allow(unused)]
    pub rnd_data_to_pattern: bool,
}

impl FuzzyConfig {
    pub fn new() -> FuzzyConfig {
        match generic::read_file("config.toml") {
            Ok(data) => toml::from_str(&data).unwrap(),
            Err(e) => panic!("config.toml problem! {:?}", e)
        }
    }
    /// fuzzing from VM reading config from .iso 
    pub fn new_from_qemu(path: String) -> FuzzyConfig {
        match generic::read_file(format!("{}/config.toml", path).as_str()) {
            Ok(data) => toml::from_str(&data).unwrap(),
            Err(e) => panic!("config.toml problem! {:?}", e),
        }
    }
}
