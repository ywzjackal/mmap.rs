#![allow(dead_code)]
extern crate libc;

pub mod linux;

use linux::MMap;
use std::io;
use std::result;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Other(String),
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone)]
pub struct RegisterMMap {
    map: MMap,
}

impl RegisterMMap {
    pub fn new(base_addr: usize, len: usize) -> Result<RegisterMMap> {
        let map = try!(MMap::with_file(0, len, "/dev/mem", base_addr).map_err(|e| Error::IO(e)));
        Ok(RegisterMMap { map: map })
    }

    pub fn get<T>(&self, offset: usize) -> T
        where T: Copy
    {
        let v: *mut T = self.map.as_object_pointer(offset);
        unsafe { *v }
    }

    pub fn set<T>(&self, offset: usize, value: T)
        where T: Copy
    {
        let v: *mut T = self.map.as_object_pointer(offset);
        unsafe { *v = value };
    }

    pub fn write<T>(&self, offset: usize, buff: &[T])
        where T: Copy
    {
        let v: *mut T = self.map.as_object_pointer(offset);
        for t in buff {
            unsafe { *v = *t };
        }
    }

    pub fn read<T>(&self, offset: usize, buff: &mut [T]) -> usize
        where T: Copy
    {
        let v: *mut T = self.map.as_object_pointer(offset);
        let mut l = 0;
        for i in 0..buff.len() {
            unsafe { buff[i] = *v };
            l = i;
        }
        l
    }
}
