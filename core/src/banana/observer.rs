use exec::call::Call;
use state::state::StateInfo;

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct WantedMask {
    pub mid: u64,
    pub uid: u64,
    pub sid: u64,
    pub cid: u64,
}

impl WantedMask {
    pub fn is_wanted(&self, uid: u64, sid: u64, cid: u64) -> bool {
        if 0 != self.uid && self.uid != uid {
            return false
        }
        if 0 != self.sid && 0 != self.sid & sid {
            return false
        }
        if 0 != self.cid && 0 != self.cid & cid { // or != instead of & ?
            return false
        }
        true
    }
}

/// (pre) callback per (sys)-call
pub trait ICallObserver {
/// - you can do arbitrary action here ( pre - callback )
/// - you can deny execution of call ( return false )
/// - it is provided immutable Call to you, so you can extract info
/// - therefore if you want to have some state of your module - use RefCell and Mutex ..
///
/// + Examples of modules :
///     - filter ( implemented 1k calls, but we want to fuzz only ( selected / random ) 60 )
///     - poclogger - you need to repro executed loops to re-trigger issue dont you ?
///     - race unlocker - by default one state is fuzzed by exact one thread - race by two same
///     state-group call unfriendly
///         - however you can create another 5 dups of this state ( 5 diff threads ), therefore race
///         friendly
///     - singlethreader -> even if SINGLE threading ( one (sys)-call at one time ) is enforced at
///     qloop, problem with w32k-callbacks may arise f.e.
///         - as once there you want to invoke another call you can not as you will deadlock
///         - avoid it you will create special artificial signal-callback to release lock on this
///         call ( is ok as currently you are in user mode )
///         - however then is problem, with race-ing and loging back, but with separate module you
///         can solve it - manage signals, and sync at this level
///     - ...
    fn notify(&self, info: &StateInfo, call: &mut Call) -> Result<bool, WantedMask>;
    fn aftermath(&self, _info: &StateInfo, _call: &mut Call) { }
    /// if some other module in stack after, denied call
    fn revert(&self, _info: &StateInfo, _call: &Call, _mask: WantedMask) { }

    fn stop(&self) { }
}
/// (pre) callback per state creation
///
/// essential for pocloging, and other stuffs, you can also deny to create specific types of states,
/// or at specific time ( state )
pub trait IStateObserver {
    /// creation of state - ability to deny to fuzz it any longer ( will be not even added to fuzzing queue )
    ///
    /// - however creation call already invoked on it, and delete call will be executed as well
    fn notify_ctor(&self, info: &StateInfo) -> bool;
    /// just to notify you, that fuzzing for this state is over
    fn notify_dtor(&self, _info: &StateInfo) { }
}
