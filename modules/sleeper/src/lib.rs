#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate rand;
use rand::Rng;

use std::{thread, time};

extern crate core;

use core::exec::call::Call;
use core::exec::id::CallTableId;
use core::banana::observer::{ICallObserver, IStateObserver};
use core::state::state::StateInfo;

use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SleeperConfig {
    target_info: BTreeMap<String, [u64; 2]>,
}
struct Sleeper {
    target_list: BTreeMap<CallTableId, u64>,
}

impl ICallObserver for Sleeper {
    fn notify(&self, state: &StateInfo, call: &Call) -> bool {
        if 0 == state.sucess {
            //ctors should be as fast as possible, we want sleep operations on states
            return true;
        }
        // well likely we want to introduce target list
        // intentionally slow down fuzzer may seems countraproductive
        // though it may add fuzzing logic 
        //   - some logic is hard to trigger
        //   - if that happen you want let object in that state little
        //   - and racers can try to do magic on that object / state
        //   - as if we let complicated logic state quickly end, we may loose some nice fuzzing
        //   setting
        if let Some(time) = self.target_list.get(&call.id()) {
          thread::sleep(time::Duration::from_millis(
              rand::thread_rng().gen_range(0..=time.clone()),
          ));
        }
        true
    }
}

impl Sleeper {
    pub(crate) fn new(cfg: &SleeperConfig) -> Sleeper {
        Sleeper { 
            target_list: cfg
                .target_info
                .iter()
                .map(|(_, [id, time])| (CallTableId::Id(id.clone()), time.clone()))
                .collect::<BTreeMap<CallTableId, u64>>(),
        }
    }
}

pub fn observers(
    cfg: &Option<SleeperConfig>,
) -> (
    Option<Box<dyn IStateObserver>>,
    Option<Box<dyn ICallObserver>>,
) {
    match *cfg {
        Some(ref cfg) => (None, Some(Box::new(Sleeper::new(&cfg)))),
        _ => (None, None),
    }
}
