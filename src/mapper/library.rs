use std::ffi::c_void;

use super::Mapper;
use load_library_rs::{Error, Library, allocators::PreAllocated};

pub struct LibraryMapper {
    allocation: *const c_void,
    len: usize,
    target_fn: Option<String>,
}

impl LibraryMapper {
    pub fn new(target: Option<&str>, allocation: *const c_void, len: usize) -> Self {
        Self {
            allocation: allocation,
            len: len,
            target_fn: target.map(|s| s.to_string()),
        }
    }
}

impl Mapper for LibraryMapper {
    type Error = Error;

    fn map(&mut self, data: &[u8]) -> Result<*const c_void, Self::Error> {
        let lib = Library::from_with_allocator(data, PreAllocated::new(self.allocation)).map()?;
        if let Some(name) = &self.target_fn {
            lib.function(&name).ok_or(Error::MissingData)
        } else {
            lib.base_address
                .map(|a| a as *const c_void)
                .ok_or(Error::InvalidMappingAddress)
        }
    }
}
