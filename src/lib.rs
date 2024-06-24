#[cfg(target_os = "linux")]
mod xlib;

#[cfg(target_os = "windows")]
mod win32;

pub mod cairo;
pub mod math;
pub mod ui;
pub mod window;
