#[files_embedded_as_modules::embed_files_as_modules(
    "
resource_folder = 'examples/resources'
"
)]
pub type Resource = &'static str;

pub fn main() {
    assert_eq!(root::_ENV, "BASE=https://example.com\n");
    assert_eq!(root::configuration::MENU_JSON, "\"Start\"\n");
    assert_eq!(root::configuration::TRANSLATIONS_CSV, "Back\n");
    assert_eq!(root::CREDITS_MD, "Foo Bar\n");
    assert_eq!(root::world::levels::TUTORIAL_JSON, "\"Hi\"\n");
    assert_eq!(root::world::PHYSICAL_CONSTANTS_JSON, "7e-3\n");
}
