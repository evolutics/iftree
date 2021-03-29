#[files_embedded_as_modules::embed_files_as_modules(
    "
resource_paths = 'examples/resources/**'
"
)]
pub type Resource = &'static str;

pub fn main() {
    use root::examples::resources;

    assert_eq!(resources::_ENV, "BASE=https://example.com\n");
    assert_eq!(resources::configuration::MENU_JSON, "\"Start\"\n");
    assert_eq!(resources::configuration::TRANSLATIONS_CSV, "Back\n");
    assert_eq!(resources::CREDITS_MD, "Foo Bar\n");
    assert_eq!(resources::world::levels::TUTORIAL_JSON, "\"Hi\"\n");
    assert_eq!(resources::world::PHYSICAL_CONSTANTS_JSON, "7e-3\n");

    assert_eq!(ARRAY[0], &resources::_ENV);
    assert_eq!(ARRAY[1], &resources::configuration::MENU_JSON);
    assert_eq!(ARRAY[2], &resources::configuration::TRANSLATIONS_CSV);
    assert_eq!(ARRAY[3], &resources::CREDITS_MD);
    assert_eq!(ARRAY[4], &resources::world::levels::TUTORIAL_JSON);
    assert_eq!(ARRAY[5], &resources::world::PHYSICAL_CONSTANTS_JSON);
}
