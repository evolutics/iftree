#[iftree::include_file_tree("resource_paths = '/examples/resources/**'")]
pub struct Resource {
    content: &'static str,
}

pub fn main() {
    use base::examples::resources;

    assert_eq!(resources::_ENV.content, "BASE=https://example.com\n");
    assert_eq!(resources::configuration::MENU_JSON.content, "\"Start\"\n");
    assert_eq!(resources::configuration::TRANSLATIONS_CSV.content, "Back\n");
    assert_eq!(resources::CREDITS_MD.content, "Foo Bar\n");
    assert_eq!(resources::world::levels::TUTORIAL_JSON.content, "\"Hi\"\n");
    assert_eq!(resources::world::PHYSICAL_CONSTANTS_JSON.content, "7e-3\n");
}
