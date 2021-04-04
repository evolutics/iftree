use std::cmp;

#[files_embedded_as_modules::embed_files_as_modules(
    "
resource_paths = 'examples/resources/**'
generate_array = true
"
)]
#[derive(cmp::PartialEq, Debug)]
pub struct Resource {
    relative_path: &'static str,
    content: &'static str,
}

pub fn main() {
    use root::examples::resources;

    assert_eq!(ARRAY.len(), 6);
    assert_eq!(ARRAY[0], &resources::_ENV);
    assert_eq!(ARRAY[1], &resources::configuration::MENU_JSON);
    assert_eq!(ARRAY[2], &resources::configuration::TRANSLATIONS_CSV);
    assert_eq!(ARRAY[3], &resources::CREDITS_MD);
    assert_eq!(ARRAY[4], &resources::world::levels::TUTORIAL_JSON);
    assert_eq!(ARRAY[5], &resources::world::PHYSICAL_CONSTANTS_JSON);

    let key_function = |resource: &&Resource| resource.relative_path;

    let index = ARRAY.binary_search_by_key(&"examples/resources/credits.md", key_function);
    assert_eq!(index, Ok(3));
    assert_eq!(ARRAY[index.unwrap()].content, "Foo Bar\n");

    assert_eq!(
        ARRAY.binary_search_by_key(&"examples/resources/seed.json", key_function),
        Err(4),
    );
}
