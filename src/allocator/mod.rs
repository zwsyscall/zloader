use std::ffi::c_void;
mod load_library;

mod virtual_alloc;
pub use virtual_alloc::NtVirtunalAlloc;

pub enum AllocationFlags {
    ReadOnly = 0x02,
    ReadWrite = 0x04,
    ExecuteRead = 0x20,
    ExecuteReadWrite = 0x40,
}

pub trait Allocator {
    type Error: std::fmt::Debug;

    fn allocate(len: usize, flags: AllocationFlags) -> Result<*const c_void, Self::Error>;
}
