macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        Asset($relative_path, include_str!($absolute_path))
    };
}

#[iftree::include_file_tree(
    "
paths = '/examples/assets/**'
template.initializer = 'initialize'
"
)]
pub struct Asset(&'static str, &'static str);

fn main() {
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
    assert_eq!(assets::configuration::TRANSLATIONS_CSV.1, "Hi {{name}}\n");

    assert_eq!(assets::CREDITS_MD.0, "examples/assets/credits.md");
    assert_eq!(assets::CREDITS_MD.1, "Boo Far\n");

    assert_eq!(
        assets::world::levels::TUTORIAL_JSON.0,
        "examples/assets/world/levels/tutorial.json",
    );
    assert_eq!(assets::world::levels::TUTORIAL_JSON.1, "\"Welcome\"\n");

    assert_eq!(
        assets::world::PHYSICAL_CONSTANTS_JSON.0,
        "examples/assets/world/physical_constants.json",
    );
    assert_eq!(assets::world::PHYSICAL_CONSTANTS_JSON.1, "7e-3\n");
}
