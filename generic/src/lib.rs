//#![feature(llvm_asm, asm, unboxed_closures)]
#![feature(unboxed_closures)]

pub mod native_alloc;

use std::{
	io,
	mem,
	ops,
	thread,
};
use std::fs::{
    File,
    OpenOptions,
};
use std::io::prelude::*;

pub fn read_file_raw(fname: &str) -> Result<Vec<u8>, io::Error> {
    let mut data = Vec::new();
    File::open(fname)?
        .read_to_end(&mut data)?;
    Ok(data)
}

pub fn read_file(fname: &str) -> Result<String, io::Error> {
    let mut data = String::new();
    File::open(fname)?
        .read_to_string(&mut data)?;
    Ok(data)
}

pub fn write_file_raw(fname: &str, data: &[u8]) -> Result<String, io::Error> {
    File::create(fname)?
        .write_all(data)?;
    Ok(String::from(fname))
}

pub fn append_file_raw(fname: &str, data: &[u8]) -> Result<String, io::Error> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(fname)?
        .write_all(data)?;
    Ok(String::from(fname))
}

pub fn data_mut_unsafe<T>(data: &mut[u8]) -> &mut T {
    if mem::size_of::<T>() > data.len() {
        panic!("trying to load complex struct of size {} vs {}", mem::size_of::<T>(), data.len());
    }
    assert!(mem::size_of::<T>() <= data.len());
    let val = unsafe { ::std::slice::from_raw_parts_mut(data.as_ptr() as *mut T, 1) };
    &mut val[0]
}

pub fn data_const_unsafe<T>(data: &[u8]) -> &T {
    if mem::size_of::<T>() > data.len() {
        panic!("trying to view complex struct of size {} vs {}", mem::size_of::<T>(), data.len());
    }
    assert!(mem::size_of::<T>() <= data.len());
    let val = unsafe { ::std::slice::from_raw_parts(data.as_ptr() as *const T, 1) };
    &val[0]
}

pub unsafe fn any_as_mut_u8_slice<T: Sized>(p: &mut T) -> &mut [u8] {
    ::std::slice::from_raw_parts_mut(
        (p as *mut T) as *mut u8,
        ::std::mem::size_of::<T>())
}
pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>())
}

use std::os::raw::c_void;

// **totaly not OK** and fcking unsafe stuffs .. you dont want to do it, unless you want ..
// usable for w32k callbacks and friends, as there we need to provide directly vtable->func_ptr
// so we wrap it to rust .. wonder how long this will be 'valid' lol, magic 3 ;)
pub unsafe fn c_callback<T>(func: T) -> usize
    where T: FnMut(u64, u32, usize, usize) -> usize
{
    let trait_obj: &dyn FnMut(u64, u32, usize, usize) -> usize = &func;
    let closure_pointer_pointer: *const c_void = std::mem::transmute(&trait_obj);
    *std::mem::transmute::<_, &usize>(
        3 * std::mem::size_of::<usize>() + *std::mem::transmute::<_, &usize>(
            std::mem::size_of::<usize>() + closure_pointer_pointer as usize))
}
pub unsafe fn c_callback_nop<T>(func: T) -> usize
    where T: FnMut() -> usize
{
    let trait_obj: &dyn FnMut() -> usize = &func;
    let closure_pointer_pointer: *const c_void = std::mem::transmute(&trait_obj);
    *std::mem::transmute::<_, &usize>(
        3 * std::mem::size_of::<usize>() + *std::mem::transmute::<_, &usize>(
            std::mem::size_of::<usize>() + closure_pointer_pointer as usize))
}

extern crate winapi;
extern crate kernel32;

use std::ffi::CString;

#[cfg(target_os = "windows")]
pub fn get_main_module() -> usize {
    unsafe { kernel32::GetModuleHandleA(0 as *mut i8) as usize }
}

#[cfg(target_os = "windows")]
pub fn get_module(module: &str) -> usize {
    unsafe { kernel32::LoadLibraryA(CString::new(module).unwrap().as_ptr()) as usize }
}
#[cfg(not(target_os = "windows"))]
pub fn get_module(_module_: &str) -> usize {
    panic!("get module is not implemented!")
}
#[cfg(target_os = "windows")]
pub fn load_api(module: &str, api: &str) -> winapi::minwindef::FARPROC {
    let addr = unsafe {
        kernel32::GetProcAddress(
            kernel32::LoadLibraryA(CString::new(module).unwrap().as_ptr()),
            CString::new(api).unwrap().as_ptr())
    };
    if 0 == addr as u64 {
//        panic!("unresolved : {} of {}", module, api);
        println!("unresolved : {} of {}", module, api);
    }
    addr
}

#[cfg(not(target_os = "windows"))]
use std::path::Path;
#[cfg(not(target_os = "windows"))]
use std::os::unix::ffi::OsStrExt;

#[cfg(not(target_os = "windows"))]
extern "C" {
    fn dlopen(filename: *const libc::c_char, flag: u32) -> *mut libc::c_void;
    fn dlsym(handle: *mut libc::c_void, symbol: *const libc::c_char) -> *mut libc::c_void;
}

#[cfg(not(target_os = "windows"))]
pub fn load_api(module: &str, api: &str) -> usize {
    const LAZY: u32 = 1;
    let path = Path::new(module);
    let cpath = CString::new(path.as_os_str().as_bytes()).unwrap();
    let capi = CString::new(api).unwrap();
    let addr = unsafe {
        std::mem::transmute(
            dlsym(
                dlopen(cpath.as_ptr(), LAZY),
                capi.as_ptr()))
    };
    if 0 == addr as u64 {
        unsafe {
        panic!("unresolved : {} of {} --> {:#?}", module, api, dlopen(cpath.as_ptr(), LAZY)); }
    }
    addr
}

/// Memcpy
///
/// Copy N bytes of memory from one location to another.
unsafe fn byte_cpy(dst: *mut u8, b: u8) {
    *dst = b;
}
pub unsafe fn c_memcpy(dst: usize, src: &[u8]) {
    for (i, b) in src.iter().enumerate() {
        byte_cpy(std::mem::transmute(dst + i), *b)
    }
}
pub unsafe fn c_memload(src: usize, dst: &mut [u8]) {
    for i in 0..dst.len() {
        dst[i] = *std::mem::transmute::<_, *const u8>(src + i);
    }
}

#[cfg(target_os = "windows")]
pub fn c_alloc(addr: usize, size: usize) -> usize {
    let mem = unsafe {
        kernel32::VirtualAlloc(
            std::mem::transmute(addr),
            size as u64,
            0x1000 | 0x2000,//MEM_COMMIT | MEM_RESERVE
            4)//PAGE_READWRITE
    };
    if 0 == mem as u64 {
        panic!("unable to allocated memory : {} of {}", addr, size);
    }
    unsafe { std::mem::transmute(mem) }
}

extern crate libc;

#[cfg(not(target_os = "windows"))]
pub fn c_alloc(addr: usize, size: usize) -> usize {
    let mem = unsafe {
        libc::mmap(
            std::mem::transmute(addr),
            size,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED | libc::MAP_POPULATE | libc::MAP_FIXED | libc::MAP_ANONYMOUS,
            -1, 0)
    };
    if 0 == mem as u64 {
        panic!("unable to allocated memory : {} of {}", addr, size);
    }
    unsafe { std::mem::transmute(mem) }
}

use std::hash::{
    Hash,
    Hasher,
};
use std::collections::hash_map::DefaultHasher;

pub fn get_tid() -> u64 {
    let mut hasher = DefaultHasher::new();
    thread::current().id().hash(&mut hasher);
    hasher.finish()
}

pub fn to_unicode(buffer: &[u8]) -> Vec<u8> {
    buffer
        .iter()
        .fold(Vec::new(), |mut b, &c| {
            b.extend_from_slice(&[c as u8, 0u8]);
            b } )
}

pub fn u8_to_str(buf: &[u8]) -> String {
    buf.iter()
        .rev()
        .map(|b| format!("{:02X}", b))
        .collect::<String>()
}

pub fn u8uni_to_str(buf: &[u8]) -> String {
    buf.iter()
        .step_by(2)
        .filter(|b| &0u8 != *b)//or 0u8 != **b
        .map(|b| *b as char)
        .collect::<String>()
}

// hmm, find time to do this w/o so much boilerplate ...
// lol seems crate for it, or maybe even in standard library .. check it
pub fn align_to<T: ops::Div + ops::Mul + Clone>(num: T, alignment: T) -> T
    where T: From< <T as ops::Div>::Output >,
          T: From< <T as ops::Mul>::Output >
{
    T::from(alignment.clone() * T::from(num / alignment))
}

#[cfg(target_os = "windows")]
pub unsafe fn mem_patch(addr: usize, data: &[u8]) {
    let mut old = winapi::winnt::PAGE_READWRITE;
    kernel32::VirtualProtect(std::mem::transmute(addr), align_to(0x1000 + data.len() as u64, 0x1000), old, &mut old);
    c_memcpy(addr, data);
    kernel32::VirtualProtect(std::mem::transmute(addr), align_to(0x1000 + data.len() as u64, 0x1000), old, &mut old);
}
#[cfg(not(target_os = "windows"))]
pub unsafe fn mem_patch(_data: &[u8]) {
    panic!("not implemented for unix - mem_patch")
}
/*
pub fn inb(port: u16) -> u8 {
    unsafe {
        let data: u8;
        llvm_asm!("in al, dx"
            : "={al}"(data)// out
            : "{dx}"(port)// in
            : // clobbers
            : "intel"// options
            );
        data
    }
}
pub fn inl(port: u16) -> u32 {
    unsafe {
        let data: u32;
        llvm_asm!("in eax, dx"
            : "={eax}"(data)// out
            : "{dx}"(port)// in
            : // clobbers
            : "intel"// options
            );
        data
    }
}
pub fn outb(port: u16, data: u8) {
    unsafe {
        llvm_asm!("out dx, al"
            : // out
            : "{dx}"(port), "{al}"(data)// in
            : // clobbers
            : "intel"// options
            );
    };
}
pub fn outw(port: u16, data: u16) {
    unsafe {
        llvm_asm!("out dx, ax"
            : // out
            : "{dx}"(port), "{ax}"(data)// in
            : // clobbers
            : "intel"// options
            );
    };
}
pub fn outl(port: u16, data: u32) {
    unsafe {
        llvm_asm!("out dx, eax"
            : // out
            : "{dx}"(port), "{eax}"(data)// in
            : // clobbers
            : "intel"// options
            );
    };
}
*/
