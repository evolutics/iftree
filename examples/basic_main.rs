#[iftree::include_file_tree("paths = '/examples/assets/**'")]
pub struct Asset {
    content: &'static str,
}

pub fn main() {
    use base::examples::assets;

    assert_eq!(assets::_ENV.content, "BASE=https://example.com\n");
    assert_eq!(assets::configuration::MENU_JSON.content, "\"Start\"\n");
    assert_eq!(assets::configuration::TRANSLATIONS_CSV.content, "Back\n");
    assert_eq!(assets::CREDITS_MD.content, "Boo Far\n");
    assert_eq!(assets::world::levels::TUTORIAL_JSON.content, "\"Hi\"\n");
    assert_eq!(assets::world::PHYSICAL_CONSTANTS_JSON.content, "7e-3\n");

    assert_eq!(ASSETS.len(), 6);
    assert_eq!(ASSETS[3].content, "Boo Far\n");
}
