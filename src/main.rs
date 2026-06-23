#![windows_subsystem = "windows"]

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

use console::DebugConsole;
use tray::TrayApp;
use winapi::um::winuser::{MessageBoxW, MB_ICONERROR, MB_OK};

mod console;
mod controller;
mod devices;
mod manager;
mod notify;
mod tray;

fn main() {
    let console = DebugConsole::new("Razer Battery Report Debug Console");

    std::env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();

    match TrayApp::new(console) {
        Ok(checker) => checker.run(),
        Err(e) => {
            let msg: Vec<u16> = OsStr::new(&format!("Error: {}", e))
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();
            let title: Vec<u16> = OsStr::new("Razer Battery Report")
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();
            unsafe {
                MessageBoxW(null_mut(), msg.as_ptr(), title.as_ptr(), MB_OK | MB_ICONERROR);
            }
            std::process::exit(1);
        }
    }
}
