#[iftree::include_file_tree(
    "
paths = '/examples/assets/**'

[field_templates]
0 = 'relative_path'
1 = 'content'
"
)]
pub struct Asset(&'static str, &'static str);

pub fn main() {
    use base::examples::assets;

    assert_eq!(assets::_ENV.0, "examples/assets/.env");
    assert_eq!(assets::_ENV.1, "BASE=https://example.com\n");

    assert_eq!(
        assets::configuration::MENU_JSON.0,
        "examples/assets/configuration/menu.json",
    );
    assert_eq!(assets::configuration::MENU_JSON.1, "\"Start\"\n");

    assert_eq!(
        assets::configuration::TRANSLATIONS_CSV.0,
        "examples/assets/configuration/translations.csv",
    );
    assert_eq!(assets::configuration::TRANSLATIONS_CSV.1, "Back\n");

    assert_eq!(assets::CREDITS_MD.0, "examples/assets/credits.md");
    assert_eq!(assets::CREDITS_MD.1, "Foo Bar\n");

    assert_eq!(
        assets::world::levels::TUTORIAL_JSON.0,
        "examples/assets/world/levels/tutorial.json",
    );
    assert_eq!(assets::world::levels::TUTORIAL_JSON.1, "\"Hi\"\n");

    assert_eq!(
        assets::world::PHYSICAL_CONSTANTS_JSON.0,
        "examples/assets/world/physical_constants.json",
    );
    assert_eq!(assets::world::PHYSICAL_CONSTANTS_JSON.1, "7e-3\n");
}
