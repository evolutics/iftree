use std::cmp;

#[files_embedded_as_modules::embed_files_as_modules(
    "
resource_paths = 'examples/resources/**'
"
)]
#[derive(cmp::PartialEq, Debug)]
pub struct Resource;

pub fn main() {
    use root::examples::resources;

    assert_eq!(resources::_ENV, Resource);
    assert_eq!(resources::configuration::MENU_JSON, Resource);
    assert_eq!(resources::configuration::TRANSLATIONS_CSV, Resource);
    assert_eq!(resources::CREDITS_MD, Resource);
    assert_eq!(resources::world::levels::TUTORIAL_JSON, Resource);
    assert_eq!(resources::world::PHYSICAL_CONSTANTS_JSON, Resource);
}
