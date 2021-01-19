use font_kit::{handle::Handle, source::SystemSource};

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
    SystemSource::new()
        .all_fonts()
        .unwrap()
}