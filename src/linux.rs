use std::io::{Result, Error};
use libc::{c_int, c_long, c_void};
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;

type Ptr = c_long;

const PROT_READ: c_int = 0x1;        /* Page can be read.  */
const PROT_WRITE: c_int = 0x2;        /* Page can be written.  */

const MAP_SHARED: c_int = 0x01;        /* Share changes.  */
const MAP_PRIVATE: c_int = 0x02;        /* Changes are private.  */
const MAP_ANONYMOUS: c_int = 0x20;

extern "C" {
    fn mmap(_addr: *mut c_void,
            _len: c_long,
            _prot: c_int,
            _flags: c_int,
            _fd: c_int,
            _offset: c_long)
            -> *mut c_void;
    fn munmap(_addr: *mut c_void, _len: c_long) -> c_int;
    fn free(_addr: *mut c_void);
}

#[derive(Clone)]
pub struct MMap {
    addr: Ptr,
    len: c_long,
    vaddr: Ptr,
}

impl MMap {
    pub fn anonymous(addr: usize, len: usize) -> Result<MMap> {
        let len = len as c_long;
        let rt = unsafe {
            mmap(addr as *mut c_void,
                 len,
                 PROT_READ | PROT_WRITE,
                 MAP_SHARED | MAP_ANONYMOUS,
                 -1,
                 0 as c_long)
        };
        if rt as c_long == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(MMap {
                addr: addr as Ptr,
                len: len,
                vaddr: rt as Ptr,
            })
        }
    }

    pub fn with_file(addr: usize, len: usize, path: &str, offset: usize) -> Result<MMap> {
        let mut opt = OpenOptions::new();
        opt.create(false).write(true).read(true).append(false);
        let f = try!(opt.open(path).map_err(|e| e));
        let len = len as c_long;
        let rt = unsafe {
            mmap(addr as *mut c_void,
                 len,
                 PROT_READ | PROT_WRITE,
                 MAP_SHARED,
                 f.as_raw_fd() as c_int,
                 offset as c_long)
        } as Ptr;
        if rt == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(MMap {
                addr: addr as Ptr,
                len: len,
                vaddr: rt as Ptr,
            })
        }
    }

    pub fn as_object_pointer<T>(&self, offset: usize) -> *mut T {
        let addr: *mut c_void = (self.vaddr as c_long + offset as c_long) as *mut c_void;
        addr as *mut T
    }

    pub fn as_pointer(&self) -> *mut c_void {
        self.vaddr as *mut c_void
    }
}

impl Drop for MMap {
    fn drop(&mut self) {
        let rt = unsafe { munmap(self.vaddr as *mut c_void, self.len) };
        if rt != 0 {
            println!("Fail to munmap:{:?}", Error::last_os_error());
        }
    }
}