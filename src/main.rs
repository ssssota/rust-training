use std::collections::HashSet;
use font_kit::{handle, source::SystemSource};

fn main() {
    let handles = SystemSource::new().all_fonts().unwrap();
    let uniqlist: HashSet<&str> = handles
        .iter()
        .map(|handle| match handle {
            handle::Handle::Path { path, .. } => path.to_str(),
            _ => None,
        })
        .filter(|path| path.is_some())
        .map(|path| path.unwrap())
        .collect();

    for (i, pathname) in uniqlist.iter().enumerate() {
        println!("{}: {}", i, pathname);
    }
}
