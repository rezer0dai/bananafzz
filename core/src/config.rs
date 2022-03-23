extern crate toml;
extern crate generic;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FuzzyConfig {
    pub version: String,
    pub noisy: bool,

    pub dead_call: f64,
    pub afl_fix_ratio: f64,

    pub ratio: usize,
    pub max_racers_count: usize,
    pub max_queue_size: usize,

    pub new_limit: usize,
    pub dup_limit: usize,

// fuzzer specifics
    #[allow(unused)]
    pub active_time: u64,
    #[allow(unused)]
    pub push_sleep: u64,
    #[allow(unused)]
    pub after_creation_sleep: u64,

    #[allow(unused)]
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

