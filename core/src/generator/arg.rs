use std::collections::HashMap;
use std::mem;

use super::leaf::IArgLeaf;

use generic::native_alloc::NativeAlloc;

use super::super::banana::bananaq::FuzzyQ;
use std::sync::Weak;

fn build_arg(atype: &str, prefix: String, postfix: String, data: &[u8]) -> String {
    atype.to_string()
        + "arg<"
        + &data.len().to_string()
        + ">{ "
        + &prefix
        + "arg<"
        + &data.len().to_string()
        + ">{ {"
        + &data
            .iter()
            .map(|u: &u8| u.to_string() + ", ")
            .collect::<String>()
        + "} }"
        + &postfix
        + "}"
}

/// final structure describing ARGUMENT for {sys/api/in-out/..}-call
pub struct Arg {
    /// name of struct, mainly for debug purposes
    name: String,
    /// data which this argument describe ( from primitve types i8..u64, up to complex structs )
    data: NativeAlloc, //Box< Vec<u8> >,
    /// generators implement how to describe our argument, should be composed mostly from primitive types
    ///
    /// - struct XXX { u8, POINT } -> two generators : U8Leaf, POINTComposite
    generator: Box<dyn IArgLeaf>,
    /// argument can be direct value, or memory pointer; diff is "" or "new " for PoC generation!
    atype: String,
}

/// base stone of calls -> its argument, describing owned data!
impl Arg {
    /// default values of data are 0x66 pattern or 0 as for NativeAlloc settings
    ///
    /// name can contain if it describes memory pointer or primitive type
    ///
    /// note : size of Arg memory depends on generator!!
    fn new(name: &str, generator: Box<dyn IArgLeaf>, atype: &str) -> Arg {
        Arg {
            name: name.to_string() + generator.name(),
            //data: Box::new(vec![0x66u8; generator.size()]),
            //data: Box::new(vec![0x0u8; generator.size()]),
            data: NativeAlloc::new(generator.size(), 0x1000usize), //align should be configurable
            generator: generator,
            atype: atype.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn data(&self) -> &[u8] {
        self.data.data()
    }
    pub fn data_mut(&mut self) -> &mut [u8] {
        self.data.data_mut()
    }

    /// api for creating primitive (i8, i16, .. u8.. u64) types
    pub fn primitive_arg(generator: Box<dyn IArgLeaf>) -> Arg {
        assert!(generator.size() <= mem::size_of::<usize>());
        if generator.size() > mem::size_of::<usize>() {
            panic!(
                "trying to mess up with primitive arg, too huge size for {} : {}",
                generator.name(),
                generator.size()
            );
        }
        Arg::new("primitive_", generator, "")
    }
    /// api for creating memory (ALPC_MESSAGE*, char[0x100], ..) types
    pub fn memory_arg(generator: Box<dyn IArgLeaf>) -> Arg {
        Arg::new("memory_", generator, "new ")
    }

    /// walk trough all generators and grab all their specifics for generating this arg
    /// and at the end append current data
    ///
    /// + why specifics ?
    ///     - think about how to PoC (seperate miniprogram) will interpret file descriptor f.e.
    ///     - that is essential to connecting various calls
    ///     - file descriptor is example, index or returned unique identifier are the same issue
    ///     - pointers inside structure -> we need alloc new memory in poc, ..
    pub fn do_serialize(&self, fd: &[u8], shared: &[u8]) -> String {
        match self
            .generator
            .serialize(self.data.data(), fd, shared)
            .iter()
            .fold(
                (String::from(""), String::from("")),
                |(pre, post), ref info| {
                    if 0 == info.prefix.len() {
                        (pre, post)
                    } else {
                        (
                            pre + &info.prefix + &info.offset.to_string() + ", ",
                            post + ")",
                        )
                    }
                },
            ) {
            (prefix, postfix) => build_arg(&self.atype, prefix, postfix, self.data.data()),
        }
    }

    /// describe ( generate ) our data
    ///
    /// - for primitive type it will do just one generation
    /// - for complex types ( memory arguments mainly ) will generate trough composite which will walk trough its leafs
    pub fn do_generate(&mut self, bananaq: &Weak<FuzzyQ>, fd: &[u8], shared: &mut [u8]) -> bool {//&mut Self {
        self.generator.generate(bananaq, self.data.data_mut(), fd, shared)
        //self
    }

    pub fn do_save_shared(&mut self, shared: &mut [u8]) {
        self.generator.save_shared(self.data.data(), shared);
    }
    pub fn mem(&self) -> Vec<u8> {
        self.generator.mem(self.data.data())
    }
    pub fn dump(&self) -> Vec<u8> {
        self.generator.dump(self.data.data())
    }
    pub fn load(
        &mut self,
        dump: &[u8],
        data: &[u8],
        prefix: &[u8], 
        fd_lookup: &HashMap<Vec<u8>, Vec<u8>>,
    ) -> Result<usize, String> {
        self.generator
            .load(self.data.data_mut(), dump, data, prefix, fd_lookup)
    }

    /// yep, little bit of unsafety, as we want to invoke calls which are basically C stuffs
    pub fn data_mut_unsafe<T>(&mut self) -> &mut T {
        if mem::size_of::<T>() > self.data.len() {
            panic!(
                "trying to load from argument '{}' complex struct of size {} vs {}",
                self.name,
                mem::size_of::<T>(),
                self.data.len()
            );
        }
        assert!(mem::size_of::<T>() <= self.data.len());
        let val = unsafe { ::std::slice::from_raw_parts_mut(self.data_mut().as_ptr() as *mut T, 1) };
        &mut val[0]
    }
    pub fn data_const_unsafe<T>(&self) -> &T {
        if mem::size_of::<T>() > self.data.len() {
            panic!(
                "trying to read from argument '{}' complex struct of size {} vs {}",
                self.name,
                mem::size_of::<T>(),
                self.data.len()
            );
        }
        assert!(mem::size_of::<T>() <= self.data.len());
        let val = unsafe { ::std::slice::from_raw_parts(self.data().as_ptr() as *const T, 1) };
        &val[0]
    }
}
