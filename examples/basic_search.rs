use std::cmp;

#[iftree::include_file_tree("paths = '/examples/resources/**'")]
#[derive(cmp::PartialEq, Debug)]
pub struct Resource {
    relative_path: &'static str,
    content: &'static str,
}

pub fn main() {
    use base::examples::resources;

    assert_eq!(ASSETS.len(), 6);
    assert_eq!(&ASSETS[0], resources::_ENV);
    assert_eq!(&ASSETS[1], resources::configuration::MENU_JSON);
    assert_eq!(&ASSETS[2], resources::configuration::TRANSLATIONS_CSV);
    assert_eq!(&ASSETS[3], resources::CREDITS_MD);
    assert_eq!(&ASSETS[4], resources::world::levels::TUTORIAL_JSON);
    assert_eq!(&ASSETS[5], resources::world::PHYSICAL_CONSTANTS_JSON);

    let key_function = |resource: &Resource| resource.relative_path;

    let index = ASSETS.binary_search_by_key(&"examples/resources/credits.md", key_function);
    assert_eq!(index, Ok(3));
    assert_eq!(ASSETS[index.unwrap()].content, "Foo Bar\n");

    assert_eq!(
        ASSETS.binary_search_by_key(&"examples/resources/seed.json", key_function),
        Err(4),
    );
}
