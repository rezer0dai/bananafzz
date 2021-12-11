use std::alloc::{alloc_zeroed, Layout, dealloc};

pub struct NativeAlloc {
    layout: Layout,
    ptr: *mut u8,
    size: usize,
}

impl Drop for NativeAlloc {
    fn drop(&mut self) {
        unsafe {
          //println!("Dropping!");
          let guard = &std::slice::from_raw_parts_mut(self.ptr, self.layout.size())[self.size..self.size+0x10];
          if 0 != guard
              .iter()
              .filter(|&b| { 1 != *b })
              .count() {
                panic!("data overwritten : {:?}", guard)
          }

            std::ptr::drop_in_place(self.ptr);
            dealloc(self.ptr, self.layout);
        }
    }
}

fn guarded_alloc(layout: Layout, size: usize) -> *mut u8 {
  unsafe {
    let ptr = alloc_zeroed(layout) as *mut u8;
    let guard = &mut std::slice::from_raw_parts_mut(ptr, layout.size())[size..size+0x10];
    guard.clone_from_slice(&[1u8; 0x10]);
    ptr
  }
}

impl NativeAlloc {
    pub fn new(
        size: usize,
        align: usize,
        ) -> NativeAlloc
    {
        match Layout::from_size_align(size + 0x10, align) {
            Ok(layout) => NativeAlloc {
                layout: layout,
                ptr: guarded_alloc(layout, size),
                size: size,
            },
            _ => {
                panic!("BAD LAYOUT size:{}, align:{} !!", size, align)
            }
        }
    }
    pub fn len(&self) -> usize {
//        assert!(self.size == self.layout.size());
//        self.layout.size()
        self.size
    }
    pub fn data_mut(&mut self) -> &mut [u8] {
//        println!("-> {:?}", self.ptr);
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr, self.size)
        }
    }
    pub fn as_ptr_mut(&mut self) -> *mut u8 {
        self.ptr
    }
    pub fn data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.ptr, self.size)
        }
    }
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr
    }
}
