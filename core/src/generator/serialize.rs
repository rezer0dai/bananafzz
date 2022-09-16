use std::collections::HashMap;

/// serialization structure for argument for further POC generation
pub struct SerializationInfo {
    /// offset : where are those data positioned in argument
    ///
    /// - in case of IArg for leafs it is 0, otherwise for IArgComposite can differ
    pub offset: usize,
    /// buffer : final representation of data which can be compiled as part of source code of POC
    pub prefix: String,
}
/// every argument must be serializable in order to reproduce program / crash in POC
pub trait ISerializableArg {
    /// take mem as data buffer of given size, and print it to String (buffer) in a way that it could be compiled later on ( c++ )
    ///
    /// - further deatils check core/generator/{leaf / composite}.rs
    ///
    /// #Example
    /// ```
    /// impl ISerializableArg for TestArg {
    ///     fn serialize(&self, _: &[u8]) -> Vec<SerializationInfo> {
    ///         vec![
    ///             SerializationInfo {
    ///                 offset : 0,
    ///                 prefix : String::from("special("),
    ///             }]
    ///     }
    /// }
    /// ```
    fn serialize(&self, _: &[u8], _: &[u8], _: &[u8]) -> Vec<SerializationInfo> {
        vec![
            SerializationInfo {
                offset : 0,
                prefix : String::from(""),
            }]
    }
    fn mem(&self, mem: &[u8]) -> Vec<u8> { mem.to_vec() }

    // dump is easy as even in ptr in argument we just fold those data
    fn dump(&self, mem: &[u8]) -> Vec<u8> {
        self.default_dump(mem)
    }    
    fn default_dump(&self, mem: &[u8]) -> Vec<u8> {
        if 0 == mem.len() {
            return vec![]
        }

        let mut sz_data = unsafe { 
            generic::any_as_u8_slice(&mem.len()).to_vec() };
        assert!(sz_data.len() == std::mem::size_of::<usize>());
        sz_data.extend(mem);
        sz_data
    }
    // here we push trough composite.rs open-ended mem + data slices, cuze ptr logic
    // we could forward exact memory slice, but we can not easily forward closed data slice
    // because of argument can contains ptr
    // content of data behind ptr is dumped into data slice and ptr leaf should extract
    // thats why we return how much data we used from data slice!
    fn default_load(&mut self, mem: &mut[u8], dump: &[u8], data: &[u8], data_load: bool) -> usize {
        let size_size = std::mem::size_of::<usize>();

        let size: usize = *generic::data_const_unsafe(dump);
        assert!(size == mem.len(), "[BFL] loading dumped data to arg goes wrong [{:X} != {:X}] aka {:X} with {:?}", 
            size, mem.len() + size_size, dump.len(), dump);
        assert!(mem.len() == data.len());

        if data_load {
            mem.copy_from_slice(&dump[size_size..][..data.len()]);
        }
        mem.len() + size_size
    }
    fn load(&mut self, mem: &mut[u8], dump: &[u8], data: &[u8], _prefix: &[u8], _fd_lookup: &HashMap<Vec<u8>,Vec<u8>>, data_load: bool) -> Result<usize, String> {
        Ok(self.default_load(mem, dump, data, data_load))
    }
}
