#[iftree::include_file_tree("paths = '/examples/assets/**'")]
pub struct Asset {
    relative_path: &'static str,
    content: &'static str,
}

pub fn main() {
    use base::examples::assets;

    assert_eq!(assets::_ENV.relative_path, "examples/assets/.env");
    assert_eq!(assets::_ENV.content, "BASE=https://example.com\n");

    assert_eq!(
        assets::configuration::MENU_JSON.relative_path,
        "examples/assets/configuration/menu.json",
    );
    assert_eq!(assets::configuration::MENU_JSON.content, "\"Start\"\n");

    assert_eq!(
        assets::configuration::TRANSLATIONS_CSV.relative_path,
        "examples/assets/configuration/translations.csv",
    );
    assert_eq!(assets::configuration::TRANSLATIONS_CSV.content, "Back\n");

    assert_eq!(
        assets::CREDITS_MD.relative_path,
        "examples/assets/credits.md",
    );
    assert_eq!(assets::CREDITS_MD.content, "Boo Far\n");

    assert_eq!(
        assets::world::levels::TUTORIAL_JSON.relative_path,
        "examples/assets/world/levels/tutorial.json",
    );
    assert_eq!(assets::world::levels::TUTORIAL_JSON.content, "\"Hi\"\n");

    assert_eq!(
        assets::world::PHYSICAL_CONSTANTS_JSON.relative_path,
        "examples/assets/world/physical_constants.json",
    );
    assert_eq!(assets::world::PHYSICAL_CONSTANTS_JSON.content, "7e-3\n");
}
