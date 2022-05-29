use super::serialize::ISerializableArg;

use super::super::banana::bananaq::FuzzyQ;
use std::sync::Weak;


/// abstraction for Argument of {sys/api/..}call
///
/// - we force user to be serializable -> to ensure backward POC compatibility
pub trait IArgLeaf : ISerializableArg {
    /// size of memory which is responsible Argument to generate
    fn size(&self) -> usize;

    /// name of argument just for debugging purposes
    fn name(&self) -> &'static str;

    /// 1. this method should be invoked every {sys/api/..}call invoked
    ///     - and only in those situations!
    ///     - never invoked Generate for other purposes!!
    ///     - (most) important for PoC generation
    /// 2. it accepts memory slice (u8) where we can write data
    ///     - every argument must have implemented this!
    ///
    /// issue here, that we want this to be private
    ///
    /// - seems it implies that it can not share common code ?
    ///       .. aka will be the same as make this private == delete it :)
    ///       .. therefore keep it like it is, safe vs unsafe variant
    ///
    /// # Example
    /// ```
    /// impl IArgLeaf for TestArg {
    ///     fn size(&self) -> usize { self.size }
    ///
    ///     fn name(&self) -> &'static str { "test-arg" }
    ///
    ///     fn generate_unsafe(&self, mem: &mut[u8], fd: &[u8]) {
    ///         for i in 0..mem.len() {
    ///             mem[i] = (rand::thread_rng().gen_range(0u8, 0xfu8) * (self.size() as u8)) as u8;
    ///         }
    ///     }
    /// }
    /// ```
    fn generate_unsafe(&mut self, bananaq: &Weak<FuzzyQ>, mem: &mut[u8], fd: &[u8], shared: &mut[u8]);

    /// wrapping GenerateImpl per Argument, to check slice length corectness!
    fn generate(&mut self, bananaq: &Weak<FuzzyQ>, mem: &mut[u8], fd: &[u8], shared: &mut[u8]) {
        if mem.len() != self.size() {
            panic!("trying to generate Argument with wrong size {} -> {} vs {}", self.name(), mem.len(), self.size());
        }
        self.generate_unsafe(bananaq, mem, fd, shared);
    }

    fn save_shared(&mut self, _mem: &[u8], _shared: &mut[u8]) { }
}
