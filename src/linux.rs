use std::io::{Result, Error};
use libc::{c_int, c_long, c_void};

const PROT_READ: c_int = 0x1;        /* Page can be read.  */
const PROT_WRITE: c_int = 0x2;        /* Page can be written.  */

const MAP_SHARED: c_int = 0x01;        /* Share changes.  */
const MAP_PRIVATE: c_int = 0x02;        /* Changes are private.  */

extern {
    fn mmap(_addr: *mut c_void, _len: c_long, _prot: c_int, _flags: c_int, _fd: c_int, _offset: c_long) -> *mut c_void;
    fn munmap(_addr: *mut c_void, _len: c_long) -> c_int;
}

pub struct MMap {
    addr: *mut c_void,
    len: c_long,
    vaddr: *mut c_void,
}

impl MMap {
    pub fn new(addr: isize, len: usize) -> Result<MMap> {
        let len = len as c_long;
        let addr = addr as *mut c_void;
        let rt = unsafe { mmap(0 as *mut c_void, len, PROT_READ | PROT_WRITE, MAP_SHARED, -1, addr as c_long) };
        if rt as c_long == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(MMap {
                addr: addr,
                len: len,
                vaddr: rt,
            })
        }
    }

    pub fn as_object<T>(&self) -> *mut T {
        self.vaddr as *mut T
    }
}

#[allow(drop_with_repr_extern)]
impl Drop for MMap {
    fn drop(&mut self) {
        let rt = unsafe { munmap(0 as *mut c_void, self.len) };
        if rt != 0 {
            println!("Fail to munmap:{:?}", Error::last_os_error());
        }
    }
}