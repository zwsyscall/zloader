use std::arch::naked_asm;
use std::os::raw::c_void;
// Windows API
use windows::Win32::Foundation::HANDLE as Handle;
use windows::Win32::System::Diagnostics::Debug::{
    CONTEXT as Context, CONTEXT_FULL_AMD64, GetThreadContext, SetThreadContext,
};

use windows::Win32::System::Threading::{
    CreateEventW, CreateThread, ResetEvent, ResumeThread, SetEvent, SuspendThread,
    THREAD_CREATION_FLAGS, TerminateThread, WaitForSingleObject,
};

#[macro_export]
macro_rules! v {
    ($val:expr) => {
        $val as *const _ as u64
    };
}

#[macro_export]
macro_rules! registers {
    ($first:expr) => {
        vec![$crate::evasion::Register::Rcx($crate::v!($first))]
    };
    ($first:expr, $second:expr) => {
        vec![
            $crate::evasion::Register::Rcx($crate::v!($first)),
            $crate::evasion::Register::Rdx($crate::v!($second)),
        ]
    };
    ($first:expr, $second:expr, $third:expr) => {
        vec![
            $crate::evasion::Register::Rcx($crate::v!($first)),
            $crate::evasion::Register::Rdx($crate::v!($second)),
            $crate::evasion::Register::R8($crate::v!($third)),
        ]
    };
    ($first:expr, $second:expr, $third:expr, $fourth:expr) => {
        vec![
            $crate::evasion::Register::Rcx($crate::v!($first)),
            $crate::evasion::Register::Rdx($crate::v!($second)),
            $crate::evasion::Register::R8($crate::v!($third)),
            $crate::evasion::Register::R9($crate::v!($fourth)),
        ]
    };
}

pub fn one_shot(
    target: *const usize,
    registers: &[Register],
    stack_args: &[u64],
) -> Result<u64, windows::core::Error> {
    let mut thread = ThreadControl::new()?;
    thread.call(target, registers, stack_args)
}

pub enum Register {
    Rax(u64),
    Rcx(u64),
    Rdx(u64),
    Rbx(u64),
    Rsp(u64),
    Rbp(u64),
    Rsi(u64),
    Rdi(u64),
    R8(u64),
    R9(u64),
    R10(u64),
    R11(u64),
    R12(u64),
    R13(u64),
    R14(u64),
    R15(u64),
}

// Fuck windows
#[repr(C, align(16))]
pub struct ThreadControl {
    hthread: Handle,
    hevent: Handle,
    // Assume that this is always up to date
    context: Context,
}

// Stupid "hack" required to fix stack alignment issues
#[repr(align(16))]
struct AlignedContext(Context);

impl ThreadControl {
    fn get_context(t_handle: Handle) -> Result<Context, windows::core::Error> {
        let mut aligned_ctx = AlignedContext(Context::default());
        aligned_ctx.0.ContextFlags = CONTEXT_FULL_AMD64;
        unsafe {
            GetThreadContext(t_handle, &mut aligned_ctx.0)?;
        }
        return Ok(aligned_ctx.0);
    }

    pub fn new() -> Result<Self, windows::core::Error> {
        // Create a suspended thread
        unsafe {
            let t_handle = CreateThread(None, 0, None, None, THREAD_CREATION_FLAGS(0x4), None)?;

            // Get the initial context
            let ctx = Self::get_context(t_handle)?;

            // Create an event
            let e_handle = CreateEventW(None, false, false, None)?;

            Ok(Self {
                hthread: t_handle,
                hevent: e_handle,
                context: ctx,
            })
        }
    }

    fn call_prelude(&mut self, return_address: u64, stack_args: &[u64]) {
        // allign stack
        self.context.Rsp &= !0xF;
        if stack_args.len() % 2 != 0 {
            self.context.Rsp -= 8;
        }

        // push each argument
        for &arg in stack_args.iter().rev() {
            self.context.Rsp -= 8;
            let rsp = self.context.Rsp as *mut u64;
            unsafe {
                std::ptr::write_volatile(rsp, return_address);
            }
        }
        // shadow space
        self.context.Rsp -= 32;

        // return
        self.context.Rsp -= 8;
        let rsp = self.context.Rsp as *mut u64;
        unsafe {
            *rsp = return_address;
        }
    }

    pub fn call(
        &mut self,
        target: *const usize,
        registers: &[Register],
        stack_args: &[u64],
    ) -> Result<u64, windows::core::Error> {
        // Assign the register values
        for reg in registers {
            match reg {
                Register::Rax(val) => self.context.Rax = *val,
                Register::Rcx(val) => self.context.Rcx = *val,
                Register::Rdx(val) => self.context.Rdx = *val,
                Register::Rbx(val) => self.context.Rbx = *val,
                Register::Rsp(val) => self.context.Rsp = *val,
                Register::Rbp(val) => self.context.Rbp = *val,
                Register::Rsi(val) => self.context.Rsi = *val,
                Register::Rdi(val) => self.context.Rdi = *val,
                Register::R8(val) => self.context.R8 = *val,
                Register::R9(val) => self.context.R9 = *val,
                Register::R10(val) => self.context.R10 = *val,
                Register::R11(val) => self.context.R11 = *val,
                Register::R12(val) => self.context.R12 = *val,
                Register::R13(val) => self.context.R13 = *val,
                Register::R14(val) => self.context.R14 = *val,
                Register::R15(val) => self.context.R15 = *val,
            }
        }

        // Save the event handle in R13
        self.context.R13 = self.hevent.0 as u64;

        // Set target resume location
        self.context.Rip = target as u64;
        let saved_rsp = self.context.Rsp;

        // Return location
        self.call_prelude(thread_harness as *const () as u64, stack_args);

        if let Err(e) = unsafe { SetThreadContext(self.hthread, &self.context) } {
            deprintln!("Error setting context: {:?}", e);
        }

        if unsafe { ResumeThread(self.hthread) == u32::MAX } {
            deprintln!("Error resuming thread")
        }

        // Now we wait
        unsafe {
            if WaitForSingleObject(self.hevent, u32::MAX).0 != 0 {
                deprintln!("WaitForSingleObject failed");
            }
            // Now the thread has signaled that it is done
            let mut ctx = loop {
                // Suspend the thread
                SuspendThread(self.hthread);
                let ctx = Self::get_context(self.hthread)?;

                // Check that we are in the waiting section of the harness
                if ctx.R14 == 0xDEADBEEF {
                    break ctx;
                }

                ResumeThread(self.hthread);
                std::thread::yield_now();
            };

            // Reset context registeries
            ctx.Rsp = saved_rsp;
            ctx.Rax = 0;
            ctx.Rcx = 0;
            ctx.Rdx = 0;
            ctx.R8 = 0;
            ctx.R9 = 0;
            ctx.R10 = 0;
            ctx.R11 = 0;
            ctx.R14 = 0;
            self.context = ctx; // Sync
            Ok(ctx.R12) // Return function return value
        }
    }
}

impl Drop for ThreadControl {
    fn drop(&mut self) {
        unsafe {
            let _ = TerminateThread(self.hthread, 2);
            let _ = windows::Win32::Foundation::CloseHandle(self.hthread);
            let _ = windows::Win32::Foundation::CloseHandle(self.hevent);
        }
    }
}

#[unsafe(naked)]
#[unsafe(link_section = ".text")]
unsafe extern "C" fn thread_harness() {
    naked_asm!(
        "
    // Save the return value of function we returned from
    mov r12, rax
    
    // EventHandle
    mov rcx, r13 
    
    // Create muh shadow stack
    sub rsp, 32

    // Set the event to have been called
    call SetEvent       
    
    // Clean up muh shadow stack
    add rsp, 32         

    // miau miau miau miau miau
    mov r14, 0xDEADBEEF

.loop:
    pause
    jmp .loop"
    )
}
