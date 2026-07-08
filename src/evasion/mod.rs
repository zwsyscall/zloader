mod file;
pub mod time;

mod thread;
pub use thread::{Register, one_shot};

pub fn quit() {
    dprintln!("[!] Goodbye");
    let _ = file::self_delete();
    std::process::exit(0)
}
