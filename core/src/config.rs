extern crate toml;
extern crate generic;

#[derive(Debug, Deserialize, Serialize)]
pub struct FuzzyConfig {
    pub version: String,
    pub noisy: bool,
    pub dead_call: usize,
    pub state_update_freq: u16,
    pub max_racers_count: usize,
    pub max_queue_size: usize,
    pub singlethread: bool,
    pub active_seconds: u64,
    pub push_sleep: u64,
    pub new_limit: usize,
    pub dup_limit: usize,
    pub ratio: usize,
    pub after_creation_sleep: u64,
    pub push_count: u64,
    pub rnd_data_to_pattern: bool,
    pub afl_fix_ratio: f64,
}

impl FuzzyConfig {
    fn new() -> FuzzyConfig {
        match generic::read_file("config.toml") {
            Ok(data) => toml::from_str(&data).unwrap(),
            Err(_) => match generic::read_file("e:/config.toml") {//fuzzing from VM reading config from .iso 
                Ok(data) => toml::from_str(&data).unwrap(),
                Err(e) => panic!("config.toml problem! {:?}", e),
            }
        }
    }
}

lazy_static! {
    pub static ref FZZCONFIG: FuzzyConfig = FuzzyConfig::new();
}
