macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        include_str!($absolute_path)
    };
}

#[iftree::include_file_tree(
    "
paths = '/examples/assets/**'
initializer = 'initialize'
"
)]
pub type Asset = &'static str;

pub fn main() {
    use base::examples::assets;

    assert_eq!(assets::_ENV, &"BASE=https://example.com\n");
    assert_eq!(assets::configuration::MENU_JSON, &"\"Start\"\n");
    assert_eq!(assets::configuration::TRANSLATIONS_CSV, &"Back\n");
    assert_eq!(assets::CREDITS_MD, &"Boo Far\n");
    assert_eq!(assets::world::levels::TUTORIAL_JSON, &"\"Hi\"\n");
    assert_eq!(assets::world::PHYSICAL_CONSTANTS_JSON, &"7e-3\n");
}
