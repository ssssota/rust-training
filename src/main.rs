#[cfg(target_os = "macos")]
mod mac;
use std::time::SystemTime;

#[cfg(target_os = "macos")]
use mac::Sys;

#[cfg(not(target_os = "macos"))]
mod notmac;
#[cfg(not(target_os = "macos"))]
use notmac::Sys;

fn main() {
    let now = SystemTime::now();
    for _ in 0..5 {
        Sys::get_all();
    }
    if let Ok(elapsed) = now.elapsed() {
        println!("{}", elapsed.as_nanos());
    }
}
