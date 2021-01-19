use font_kit::{handle::Handle, sources::fs::FsSource};

fn main() {
    println!("Hello, world!");

    let fonts = get_fonts();
    let size = fonts.len();
    println!("size: {}", size);
    for (_i, font) in fonts.iter().enumerate() {
        match font {
            Handle::Path { path, font_index } => println!("{}, {:?}", font_index, path.to_str()),
            Handle::Memory { .. } => {}
        }
    }
}

fn get_fonts() -> Vec<Handle> {
    FsSource::new()
        .all_fonts()
        .unwrap()
}
