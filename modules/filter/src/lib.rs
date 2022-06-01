#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate core;

use core::exec::call::Call;
use core::exec::id::CallTableId;
use core::banana::observer::{ICallObserver, IStateObserver, WantedMask};
use core::state::state::StateInfo;

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Deserialize, Serialize)]
pub struct FilterConfig {
    whitelist: BTreeMap<String, u64>,
}
struct Filter {
    whiteset: BTreeSet<CallTableId>,
}

impl ICallObserver for Filter {
    fn notify(&self, _: &StateInfo, call: &mut Call) -> Result<bool, WantedMask> {
        if self.whiteset.contains(&call.id()) {
          //println!("passtrough! : {:?} -> {}", call.id(), call.name());
        }
        if self.whiteset.contains(&call.id()) {
            Ok(true)
        } else { Err(WantedMask::default()) }
    }
}

impl Filter {
    pub(crate) fn new(cfg: &FilterConfig) -> Filter {
        Filter {
            whiteset: cfg
                .whitelist
                .iter()
                .map(|(_, id)| CallTableId::Id(*id))
                .collect::<BTreeSet<CallTableId>>(),
        }
    }
}

pub fn observers(
    cfg: &Option<FilterConfig>,
) -> (
    Option<Box<dyn IStateObserver>>,
    Option<Box<dyn ICallObserver>>,
) {
    match *cfg {
        Some(ref cfg) => (None, Some(Box::new(Filter::new(&cfg)))),
        _ => (None, None),
    }
}
