#[macro_use]
extern crate serde_derive;

extern crate toml;

extern crate rand;
use rand::Rng;

extern crate generic;

use std::io;

extern crate core;
use core::exec::fd_info::Fd;
use core::banana::observer::{ICallObserver, IStateObserver};
use core::state::id::StateTableId;

extern crate common;
use common::ModuleCallbacks;

pub mod callbacks;
use callbacks::PlugCallbacks;

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
use libmediator::MediatorConfig;

#[derive(Debug, Deserialize, Serialize)]
struct ConfigCore {
    filter: Option<FilterConfig>,
    raceunlock: Option<RaceUnlockConfig>,
    sleeper: Option<SleeperConfig>,
    limiter: Option<LimiterConfig>,
    debug: Option<DebugConfig>,
    mediator: Option<MediatorConfig>,
}
#[derive(Debug, Deserialize, Serialize)]
struct Config {
    online: Vec<String>,
    core: ConfigCore,
}

//will panic if no correct crafted config!
fn load_cfg() -> Result<Config, io::Error> {
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
        println!(
            "pluging {} => state={} , call={}",
            self.name,
            self.obs.0.is_some(),
            self.obs.1.is_some()
        );
    }
}

/// control structure for installing plugins
struct Plugins {
    observers: Vec<Observer>,
}

impl Plugins {
    fn new<F>(cfg: Config, push_state: &'static F) -> Plugins
    where
        F: Fn(StateTableId, &Fd) + std::marker::Sync + std::marker::Send,
    {
        Plugins {
            observers: cfg
                .online
                .iter()
                .map(|module| Plugins::load_observer(module, &cfg.core, push_state))
                .collect(),
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
    fn load_observer<F>(module: &String, cfg: &ConfigCore, push_state: &'static F) -> Observer
    where
        F: Fn(StateTableId, &Fd) + std::marker::Sync + std::marker::Send,
    {
        let qcallbacks = Box::new(PlugCallbacks::new(push_state));
        match module.as_str() {
            "libraceunlock" => Observer {
                name: module.clone(),
                obs: libraceunlocker::observers(&cfg.raceunlock, qcallbacks.clone()),
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
                obs: liblimiter::observers(&cfg.limiter, qcallbacks.clone()),
            },
            "libdebug" => Observer {
                name: module.clone(),
                obs: libdebug::observers(&cfg.debug),
            },
            "libmediator" => Observer {
                name: module.clone(),
                obs: libmediator::observers(&cfg.mediator),
            },
            _ => Observer {
                name: module.clone(),
                obs: (None, None),
            },
        }
    }
}

pub fn plug<F>(push_state: &'static F) -> Result<Vec<Observer>, io::Error>
where
    F: Fn(StateTableId, &Fd) + std::marker::Sync + std::marker::Send,
{
    match load_cfg() {
        Ok(cfg) => Ok(Plugins::new(cfg, push_state).observers),
        Err(e) => return Err(e),
    }
}

fn bad_design(_: StateTableId, _: &Fd) {}

pub fn stop_fuzzing() {
    // problem is that plugins need this in some generic way
    // and also plugcallbacks need access to plugin global stuffs ( stop, log, .. )
    // and there we are with main.rs to stop fuzzing by generic way and PlugCallbacks beeing trait
    // TODO : maybe to rethink stop_fuzzing mechanism
    PlugCallbacks::new(&bad_design).stop_fuzzing()
}
