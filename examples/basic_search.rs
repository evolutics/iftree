use std::cmp;

#[iftree::include_file_tree("paths = '/examples/assets/**'")]
#[derive(cmp::PartialEq, Debug)]
pub struct Asset {
    relative_path: &'static str,
    content: &'static str,
}

pub fn main() {
    use base::examples::assets;

    assert_eq!(ASSETS.len(), 6);
    assert_eq!(&ASSETS[0], assets::_ENV);
    assert_eq!(&ASSETS[1], assets::configuration::MENU_JSON);
    assert_eq!(&ASSETS[2], assets::configuration::TRANSLATIONS_CSV);
    assert_eq!(&ASSETS[3], assets::CREDITS_MD);
    assert_eq!(&ASSETS[4], assets::world::levels::TUTORIAL_JSON);
    assert_eq!(&ASSETS[5], assets::world::PHYSICAL_CONSTANTS_JSON);

    let key_function = |asset: &Asset| asset.relative_path;

    let index = ASSETS.binary_search_by_key(&"examples/assets/credits.md", key_function);
    assert_eq!(index, Ok(3));
    assert_eq!(ASSETS[index.unwrap()].content, "Boo Far\n");

    assert_eq!(
        ASSETS.binary_search_by_key(&"examples/assets/seed.json", key_function),
        Err(4),
    );
}
