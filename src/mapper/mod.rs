use std::ffi::c_void;

mod library;
pub use library::LibraryMapper;
mod shellcode;
pub use shellcode::ShellcodeMapper;

pub trait Mapper {
    type Error: std::fmt::Debug;

    fn map(&mut self, data: &[u8]) -> Result<*const c_void, Self::Error>;
}
