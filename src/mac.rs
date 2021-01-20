use core_text::{font_collection::{create_for_all_families, get_family_names}, font_manager::CTFontManagerCopyAvailableFontURLs};

pub struct Sys;
impl Sys {
    pub fn get_all() {
        if let Some(descriptors) = create_for_all_families().get_descriptors() {
            descriptors.iter()
                .map(|descriptor| descriptor.font_path())
                .filter(|path| path.is_some())
                .map(|path| path?.to_str())
                .for_each(|path| println!("{}", path));
        }
    }
}
