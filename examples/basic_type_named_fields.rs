#[iftree::include_file_tree("resource_paths = '/examples/resources/**'")]
pub struct Resource {
    relative_path: &'static str,
    content: &'static str,
}

pub fn main() {
    use base::examples::resources;

    assert_eq!(resources::_ENV.relative_path, "examples/resources/.env");
    assert_eq!(resources::_ENV.content, "BASE=https://example.com\n");

    assert_eq!(
        resources::configuration::MENU_JSON.relative_path,
        "examples/resources/configuration/menu.json",
    );
    assert_eq!(resources::configuration::MENU_JSON.content, "\"Start\"\n");

    assert_eq!(
        resources::configuration::TRANSLATIONS_CSV.relative_path,
        "examples/resources/configuration/translations.csv",
    );
    assert_eq!(resources::configuration::TRANSLATIONS_CSV.content, "Back\n");

    assert_eq!(
        resources::CREDITS_MD.relative_path,
        "examples/resources/credits.md",
    );
    assert_eq!(resources::CREDITS_MD.content, "Foo Bar\n");

    assert_eq!(
        resources::world::levels::TUTORIAL_JSON.relative_path,
        "examples/resources/world/levels/tutorial.json",
    );
    assert_eq!(resources::world::levels::TUTORIAL_JSON.content, "\"Hi\"\n");

    assert_eq!(
        resources::world::PHYSICAL_CONSTANTS_JSON.relative_path,
        "examples/resources/world/physical_constants.json",
    );
    assert_eq!(resources::world::PHYSICAL_CONSTANTS_JSON.content, "7e-3\n");
}
