use core_text::font_collection::create_for_all_families;

pub struct Sys;
impl Sys {
    pub fn get_all() {
        if let Some(descriptors) = create_for_all_families().get_descriptors() {
            descriptors.iter()
                .for_each(|d| println!("{}", d.font_path().unwrap().to_str().unwrap()))
        }
    }
}
