#[cfg(target_os = "macos")]
mod mac;
#[cfg(target_os = "macos")]
use mac::Sys;

#[cfg(not(target_os = "macos"))]
mod notmac;
#[cfg(not(target_os = "macos"))]
use notmac::Sys;

fn main() {
    for _ in 0..5 {
        Sys::get_all();
    }
}
