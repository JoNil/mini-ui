#[cfg(target_os = "linux")]
pub mod posix;
#[cfg(target_os = "windows")]
pub mod windows;
