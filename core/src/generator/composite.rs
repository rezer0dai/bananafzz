use std::ops::Range;
use std::collections::HashMap;

use super::leaf::IArgLeaf;
use super::serialize::{
    ISerializableArg, 
    SerializationInfo
};

use super::super::banana::bananaq::FuzzyQ;
use std::sync::Weak;

/// structure capable of describing any argument, composed of IArgLeafs
///
/// - mostly any custom argument declared for syscall should implement this!
///     - creating own leafs should be discouraged, however may be sometimes needed
pub struct ArgComposite
{
    /// size of argument described;
    ///
    /// - not necessary all memory of composite is described by leafs ( unitialized - pattern by default)
    size: usize,
    /// unique name for this; primarly for debug purposes
    name: &'static str,
    /// array of leafs, every leaf describing own subpart of argument described by composite
    ///
    /// - Const::new8, StrLeaf, RandomData, ...
    /// - complex structure passing to call is composed of small primitive types
    /// - composite groups them together and generate per demand
    args: Vec<(usize, Box<dyn IArgLeaf>)>,

    logicer: Option<Box<dyn IArgLeaf>>,
}

//O(n**2) algo, but it is ok as N is very small and we do it only once ...
fn sanitize_overlaping(args: &[(usize, Box<dyn IArgLeaf>)], size: usize) -> bool {
    args
        .iter()
        .enumerate()
        .any(|(ind, arg_couple)| {
            let range: Range<usize> = arg_couple.0..arg_couple.0+arg_couple.1.size();
            if range.end > size {
                panic!("ArgComposite oversized with leafs {} vs {}", size, range.end);
            }
            args[ind+1..]
                .iter()
                .any(|&(start, ref arg)| {
                    if range.start >= start + arg.size() || range.end <= start {
                        false
                    } else {
                        println!("overlap in [ {} vs {} ] => {} {} -> {} {}",
                                arg_couple.1.name(), arg.name(),
                                range.start, range.end,
                                start, start + arg.size());
                        true
                    }
                })
        })
}

/// default implements only ctor for struct
impl ArgComposite {
    /// - elegant feature, is that arguments can be in arbitrary order ( therefore offset needed )
    /// - they will be built in order you specify
    ///     - can be nice, once you have corelation between two sub-args, and once is dependable on another but in struct those are in reversed order ...
    ///
    /// # Panic
    /// - overlaping is forbiden (paniced)!
    ///
    /// # Example :
    /// ```
    /// impl ArgComposite {
    /// pub fn test_arg(size: usize) -> ArgComposite {
    ///     ArgComposite::new(
    ///         size,
    ///         "TestArg-composite",
    ///         vec![
    ///             (0, Box::new(TestArg::new(1))),
    ///             (1, Box::new(TestArg::new(size - 1))),
    ///         ])
    ///     }
    /// }
    /// ```
    pub fn new(
        size: usize,
        name: &'static str,
        args: Vec<(usize, Box<dyn IArgLeaf>)>
        ) -> ArgComposite
    {
        if sanitize_overlaping(&args, size) {
            panic!("overlap in {}", name)
        }

        ArgComposite {
            size : size,
            name : name,
            args : args,
            logicer : None,
        }
    }
    pub fn new_w_logic(
        size: usize,
        name: &'static str,
        args: Vec<(usize, Box<dyn IArgLeaf>)>,
        logicer: Box<dyn IArgLeaf>,
        ) -> ArgComposite
    {
        if sanitize_overlaping(&args, size) {
            panic!("overlap in {}", name)
        }

        ArgComposite {
            size : size,
            name : name,
            args : args,
            logicer : Some(logicer),
        }
    }
}

impl IArgLeaf for ArgComposite {
    fn size(&self) -> usize { self.size }

    fn name(&self) -> &'static str { self.name }

    //better if this will be private ( different trait dont used in queue )
    fn generate_unsafe(&mut self, bananaq: &Weak<FuzzyQ>, mem: &mut[u8], fd: &[u8], shared: &mut[u8]) -> bool {
        // for &(off, ref mut arg) in self.args.iter() {
        //     let size = arg.size();
        //     arg.generate(&mut mem[off..off+size])
        // }
        // while let Some((off, ref mut arg)) = self.args.in_iter().next() {
        // }
        for i in 0..self.args.len() {
            let (off, ref mut arg) = self.args[i];
            let size = arg.size();
            if !arg.generate(bananaq, &mut mem[off..off+size], fd, shared) {
                return false
            }
        }

        if let Some(ref mut logicer) = &mut self.logicer {
            if !logicer.generate_unsafe(bananaq, mem, fd, shared) {
                return false
            }
        }//we could use function instead of IArgLeaf, but likely we want to the same with "load" in the future
        true
    }
    fn save_shared(&mut self, mem: &[u8], shared: &mut[u8]) { 
        for i in 0..self.args.len() {
            let (off, ref mut arg) = self.args[i];
            let size = arg.size();
            arg.save_shared(&mem[off..off+size], shared)
        }
    }
}

/// default serialization provider
impl ISerializableArg for ArgComposite {
    /// 1. we want to walk trough all aruments
    /// 2. serialize all of them
    /// 3. update their particular offset ( as those are non-overlapped and together build argument )
    /// 4. forward it to caller
    ///
    /// # Example
    /// ```
    /// impl ISerializableArg for TestArg { }
    /// ```
    /// ```
    /// fn serialize(&self, mem: &[u8], fd: &[u8]) -> Vec<SerializationInfo> {
    ///     vec![
    ///         SerializationInfo {
    ///             offset : 0,
    ///             prefix : String::from("special("),
    ///         }]
    /// }
    /// ```
    fn serialize(&self, mem: &[u8], fd: &[u8], shared: &[u8]) -> Vec<SerializationInfo> {
        self.args
            .iter()
            .map(|&(off, ref arg)|
                 arg.serialize(&mem[off..off+arg.size()], fd, shared)
                    .into_iter()
                    .map(move |mut info| {
                        info.offset += off;
                        info })
                    .collect::< Vec<SerializationInfo> >()
            )
            .flat_map(move |info| info)
            .collect::< Vec<SerializationInfo> >()
        /*
        //imperative alternative, tested for speed and #lines; FP faster and just bit longer + no
        //temporary variable!
        let mut infos: Vec<SerializationInfo> = Vec::new();
        for &(off, ref arg) in self.args.iter() {
            for mut info in arg.serialize(&mem[off..off+arg.size()]).into_iter() {
                info.offset += off;
                infos.push(info);
            }
        }
        infos
        */
    }

    fn mem(&self, mem: &[u8]) -> Vec<u8> {
        self.args
            .iter()
            .map(|&(off, ref arg)| {
                arg.mem(&mem[off..off+arg.size()])
            })
            .flat_map(move |data| data)
            .collect::< Vec<u8> >()
    }

    fn dump(&self, mem: &[u8]) -> Vec<u8> {
        self.args
            .iter()
            .map(|&(off, ref arg)| {
                arg.dump(&mem[off..off+arg.size()])
            })
            .flat_map(move |data| data)
            .collect::< Vec<u8> >()
    }
    fn load(&mut self, mem: &mut[u8], dump: &[u8], data: &[u8], prefix: &[u8], fd_lookup: &HashMap<Vec<u8>,Vec<u8>>, data_load: bool) -> Result<usize, String> {
        let mut off_d = 0;
        for &mut (off, ref mut arg) in self.args.iter_mut() {
            let asize = arg.size();

            let size = arg.load(&mut mem[off..][..asize], 
                &dump[off_d..], &data[off..][..asize], prefix, fd_lookup, data_load)?;

            off_d += size;
        }
//        if let Some(ref mut logicer) = &mut self.logicer {
//            logicer.load(mem, dump, data, prefix, fd_lookup);
//        }
        Ok(off_d)
    }
}
