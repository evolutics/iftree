use std::cmp;

#[iftree::include_file_tree("paths = '/examples/assets/**'")]
#[derive(cmp::PartialEq, Debug)]
pub struct Asset;

fn main() {
    use base::examples::assets;

    assert_eq!(assets::_ENV, &Asset);
    assert_eq!(assets::configuration::MENU_JSON, &Asset);
    assert_eq!(assets::configuration::TRANSLATIONS_CSV, &Asset);
    assert_eq!(assets::CREDITS_MD, &Asset);
    assert_eq!(assets::world::levels::TUTORIAL_JSON, &Asset);
    assert_eq!(assets::world::PHYSICAL_CONSTANTS_JSON, &Asset);
}
