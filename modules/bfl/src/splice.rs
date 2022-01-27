use poc::{
    ShmemId,
    PocData, 
    BananizedFuzzyLoopConfig,
};

pub struct Splice {
    cfg: BananizedFuzzyLoopConfig,
}

impl Splice {
    pub fn new(config: &BananizedFuzzyLoopConfig) -> Splice {
//load splice memory fo signal if splice is wanted or not
        Splice {
            cfg : *config,
        }
    }
    pub fn process(&mut self) {
//check loaded spliced memory, if splice is wanted, else return

        let mut poc_a = PocData::new(&self.cfg, ShmemId::SpliceA);
        if poc_a.header().magic != self.cfg.magic {
            return//not our call
        }

        let mut poc_b = PocData::new(&self.cfg, ShmemId::SpliceB);
        if poc_b.header().magic != self.cfg.magic {
            return//not our call
        }

        let mut poc_o = PocData::new(&self.cfg, ShmemId::PocOut);

        for i in 0..poc_a.header().insert_ind {
            poc_o.append(poc_a.call(i), poc_a.desc(i).kin);
        }
        for i in poc_b.header().insert_ind..poc_b.header().calls_count {
            poc_o.append(poc_b.call(i), poc_b.desc(i).kin);
        }

        poc_a.discard();
        poc_b.discard();

        poc_o.share(); 
    }
}
