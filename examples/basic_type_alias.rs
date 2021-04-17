#[iftree::include_file_tree(
    "
paths = '/examples/assets/**'

[field_templates]
_ = 'content'
"
)]
pub type Asset = &'static str;

pub fn main() {
    use base::examples::assets;

    assert_eq!(assets::_ENV, &"BASE=https://example.com\n");
    assert_eq!(assets::configuration::MENU_JSON, &"\"Start\"\n");
    assert_eq!(assets::configuration::TRANSLATIONS_CSV, &"Back\n");
    assert_eq!(assets::CREDITS_MD, &"Foo Bar\n");
    assert_eq!(assets::world::levels::TUTORIAL_JSON, &"\"Hi\"\n");
    assert_eq!(assets::world::PHYSICAL_CONSTANTS_JSON, &"7e-3\n");
}
