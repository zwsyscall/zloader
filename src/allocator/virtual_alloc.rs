use std::{ffi::c_void, ptr::null_mut};

use super::{AllocationFlags, Allocator};

unsafe extern "system" {
    fn GetLastError() -> u32;

    fn VirtualAlloc(
        address: *mut c_void,
        size: usize,
        alloc_type: u32,
        protect_flags: u32,
    ) -> *const c_void;
}

pub struct NtVirtunalAlloc {}

impl Allocator for NtVirtunalAlloc {
    type Error = u32;

    fn allocate(len: usize, flags: AllocationFlags) -> Result<*const c_void, Self::Error> {
        unsafe {
            let addr = VirtualAlloc(null_mut(), len, 0x1000 | 0x2000, flags as u32);

            match addr.is_null() {
                false => return Ok(addr),
                true => return Err(GetLastError()),
            }
        }
    }
}
