
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi;
use std::os::raw::{c_int, c_void};

struct mapping {
    name: String,
    handle: c_int,
    mapping: *mut c_void,

}

impl mapping {
    pub fn new(name: &str, size: usize) -> Self {
        let handle;
        let p;

        println!("called new");
        let cname = ffi::CString::new(name).unwrap();
        unsafe {
            let flags : c_int = O_RDWR as c_int | O_CREAT as c_int;

            handle = shm_open(cname.as_ptr(), flags, 0600);
            if handle == -1 {
                panic!("shm_open failed");
            }
            if ftruncate(handle, size as off_t) == -1 {
                panic!("unable to set shm size");
            }
            p = mmap(std::ptr::null_mut(), size, PROT_READ as c_int | PROT_WRITE as c_int, MAP_SHARED as c_int, handle, 0);
            if p == std::ptr::null_mut() {
                panic!("unable to mmap");
            }
        };
        mapping{ name: name.to_string(), handle: handle, mapping: p }
    }
}

impl Drop for mapping {
    fn drop(&mut self) {
        if self.handle == -1 {
            return
        }
        println!("Called drop on mapping");
        unsafe {
            let cname = ffi::CString::new(self.name.as_str()).unwrap();
            shm_unlink(cname.as_ptr());
        }
        self.handle = -1;
    }
}

fn main() {
    let m = mapping::new("test", 100000);
    println!("mapping created");
}
