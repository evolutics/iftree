#[iftree::include_file_tree(
    "
paths = '/examples/resources/**'

[field_templates]
_ = 'content'
"
)]
pub type Resource = &'static str;

pub fn main() {
    use base::examples::resources;

    assert_eq!(resources::_ENV, &"BASE=https://example.com\n");
    assert_eq!(resources::configuration::MENU_JSON, &"\"Start\"\n");
    assert_eq!(resources::configuration::TRANSLATIONS_CSV, &"Back\n");
    assert_eq!(resources::CREDITS_MD, &"Foo Bar\n");
    assert_eq!(resources::world::levels::TUTORIAL_JSON, &"\"Hi\"\n");
    assert_eq!(resources::world::PHYSICAL_CONSTANTS_JSON, &"7e-3\n");
}
