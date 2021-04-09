use std::cmp;

#[iftree::include_file_tree("resource_paths = 'examples/resources/**'")]
#[derive(cmp::PartialEq, Debug)]
pub struct Resource;

pub fn main() {
    use base::examples::resources;

    assert_eq!(resources::_ENV, Resource);
    assert_eq!(resources::configuration::MENU_JSON, Resource);
    assert_eq!(resources::configuration::TRANSLATIONS_CSV, Resource);
    assert_eq!(resources::CREDITS_MD, Resource);
    assert_eq!(resources::world::levels::TUTORIAL_JSON, Resource);
    assert_eq!(resources::world::PHYSICAL_CONSTANTS_JSON, Resource);
}
