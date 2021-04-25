use std::vec;

#[iftree::include_file_tree("paths = '/examples/assets/**'")]
pub struct Asset {
    relative_path: &'static str,
    contents_str: &'static str,
}

pub fn main() {
    let key_function = |asset: &Asset| asset.relative_path;

    assert_eq!(
        ASSETS.iter().map(key_function).collect::<vec::Vec<_>>(),
        vec![
            "examples/assets/.env",
            "examples/assets/configuration/menu.json",
            "examples/assets/configuration/translations.csv",
            "examples/assets/credits.md",
            "examples/assets/world/levels/tutorial.json",
            "examples/assets/world/physical_constants.json",
        ],
    );

    let index = ASSETS.binary_search_by_key(&"examples/assets/credits.md", key_function);
    assert_eq!(index, Ok(3));
    assert_eq!(ASSETS[index.unwrap()].contents_str, "Boo Far\n");

    assert_eq!(
        ASSETS.binary_search_by_key(&"examples/assets/seed.json", key_function),
        Err(4),
    );
}
