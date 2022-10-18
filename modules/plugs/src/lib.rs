#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

extern crate toml;

extern crate rand;
use rand::Rng;

extern crate generic;

use std::io;

extern crate core;
use core::banana::observer::{ICallObserver, IStateObserver};

extern crate libsyncer;

extern crate libfilter;
use libfilter::FilterConfig;

extern crate libraceunlocker;
use libraceunlocker::RaceUnlockConfig;

extern crate libsleeper;
use libsleeper::SleeperConfig;

extern crate liblimiter;
use liblimiter::LimiterConfig;

extern crate libdebug;
use libdebug::DebugConfig;

extern crate libmediator;

extern crate libsolback;
extern crate libbehavior;
use libbehavior::BehaviorConfig;

extern crate libbfl;
use libbfl::BananizedFuzzyLoopConfig;

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigCore {
    syncer: Option<()>,
    filter: Option<FilterConfig>,
    raceunlock: Option<RaceUnlockConfig>,
    sleeper: Option<SleeperConfig>,
    pub limiter: Option<LimiterConfig>,
    debug: Option<DebugConfig>,
    mediator: Option<()>,
    solback: Option<()>,
    behavior: Option<BehaviorConfig>,
    pub bfl: Option<BananizedFuzzyLoopConfig>, //lets share only what we know we need
    smb: Option<()>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub online: Vec<String>,
    pub core: ConfigCore,
}

//will panic if no correct crafted config!
pub fn load_cfg() -> Result<Config, io::Error> {
    match generic::read_file("modules.toml") {
        Ok(data) => Ok(toml::from_str(&data).unwrap()),
        Err(_) => match generic::read_file(
            &(String::from("e:/filters/modules")
                + &rand::thread_rng().gen_range(0..2000u16).to_string()
                + ".toml"),
        ) {
            Ok(data) => Ok(toml::from_str(&data).unwrap()),
            Err(_) => match generic::read_file("e:/modules.toml") {
                Ok(data) => Ok(toml::from_str(&data).unwrap()),
                Err(e) => return Err(e),
            },
        },
    }
}

pub struct Observer {
    pub name: String,
    pub obs: (
        Option<Box<dyn IStateObserver>>,
        Option<Box<dyn ICallObserver>>,
    ),
}

impl Observer {
    pub fn state_obs(&mut self) -> &mut Option<Box<dyn IStateObserver>> {
        &mut self.obs.0
    }
    pub fn call_obs(&mut self) -> &mut Option<Box<dyn ICallObserver>> {
        &mut self.obs.1
    }
    pub fn stats(&self) {
        debug!(
            "pluging {} => state={}, call={}",
            self.name,
            self.obs.0.is_some(),
            self.obs.1.is_some(),
        );
    }
}

/// control structure for installing plugins
pub struct Plugins {
    pub observers: Vec<Observer>,
    pub cfg: Config,
}

impl Plugins {
    pub fn new(cfg: Config) -> Plugins {
        Plugins {
            observers: cfg
                .online
                .iter()
                .map(|module| Plugins::load_observer(module, &cfg.core))
                .collect(),
            cfg: cfg,
        }
    }
    /// anytime new plugin is added must be inserted loading-code here
    ///
    /// - in the future this may be separete dll for better decoupling
    ///     - adding modules add-hot style ( we just replace/add module but fuzzer remains same )
    ///     - also thats why we forwarding push_state as static function..
    /// - optimal will be, that if new module added no need to recompile fuzzer
    ///     - aka i can use 3 years old fuzzer with latest new module
    ///     - though if it really brings some benefits, loosing sources to fuzzer and use it later
    ///     is problem in its own sense
    fn load_observer(module: &String, cfg: &ConfigCore) -> Observer {
        match module.as_str() {
            "libraceunlock" => Observer {
                name: module.clone(),
                obs: libraceunlocker::observers(&cfg.raceunlock),
            },
            "libsyncer" => Observer {
                name: module.clone(),
                obs: libsyncer::observers(),
            },
            "libfilter" => Observer {
                name: module.clone(),
                obs: libfilter::observers(&cfg.filter),
            },
            "libsleeper" => Observer {
                name: module.clone(),
                obs: libsleeper::observers(&cfg.sleeper),
            },
            "liblimiter" => Observer {
                name: module.clone(),
                obs: liblimiter::observers(&cfg.limiter),
            },
            "libdebug" => Observer {
                name: module.clone(),
                obs: libdebug::observers(&cfg.debug),
            },
            "libmediator" => Observer {
                name: module.clone(),
                obs: libmediator::observers(),
            },
            "libsolback" => Observer {
                name: module.clone(),
                obs: libsolback::observers(),
            },
            "libbehavior" => Observer {
                name: module.clone(),
                obs: libbehavior::observers(&cfg.behavior),
            },
            "libbfl" => Observer {
                name: module.clone(),
                obs: libbfl::observers(&cfg.bfl),
            },
            _ => Observer {
                name: module.clone(),
                obs: (None, None),
            },
        }
    }
}

pub fn plug() -> Result<Plugins, io::Error> {
    match load_cfg() {
        Ok(cfg) => Ok(Plugins::new(cfg)),
        Err(e) => return Err(e),
    }
}
