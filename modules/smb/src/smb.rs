#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct SuperMarioBros2Config { }

pub struct SuperMarioBros2 {
    cfg: SuperMarioBros2Config,
    theone: u64,
}

impl SuperMarioBros2 {
    pub fn new(cfg: &SuperMarioBros2Config) -> Self {
        SuperMarioBros2 {
            cfg: cfg,
            theone: 0,
        }
    }
    fn notify_locked(&mut self, state: &StateInfo) -> bool {
        if state.uid == self.theone {
            return true
        }
        if 0 == state.level {
            return true//ctors may pass through by default
        }
        if 1 != state.level {
            return false//we wait for leader at first!
        }
        self.theone = state.uid;
        true
    }
    fn aftermath_locked(&mut self, state: &StateInfo) {
        assert!(state.uid == self.theone, 
            "[SMB2] aftermath with different target {:?} vs {:?}",
            state.uid, self.theone);

        if 1 != state.level {
            return//reset it when we made a move and we are bck at lead_pos level
        }
        state.uid = 0
    }
}
