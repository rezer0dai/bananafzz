extern crate core;
use self::core::generator::leaf::IArgLeaf;
use self::core::generator::serialize::ISerializableArg;

pub const POS_SIZE: usize = (4+1)*2;

pub struct MoveArg {
    size: usize,
}

impl MoveArg {
    pub fn new() -> MoveArg {
        MoveArg {
            size : 3,
        }
    }
}

impl ISerializableArg for MoveArg { }

impl IArgLeaf for MoveArg {
    fn size(&self) -> usize { self.size }

    fn name(&self) -> &'static str { "MoveArg" }

    fn generate_unsafe(&mut self, mem: &mut[u8], _: &[u8], _position: &[u8]) {
//        position
        //calculate pos..data_const_unsafe()->mario_pos - ..->target_pos
        mem.fill(3)//temporary
    }
}
