#[files_embedded_as_modules::embed_files_as_modules(
    "
resource_paths = 'examples/resources/**'

[field_templates]
0 = 'relative_path'
1 = 'content'
"
)]
pub struct Resource(&'static str, &'static str);

pub fn main() {
    use base::examples::resources;

    assert_eq!(resources::_ENV.0, "examples/resources/.env");
    assert_eq!(resources::_ENV.1, "BASE=https://example.com\n");

    assert_eq!(
        resources::configuration::MENU_JSON.0,
        "examples/resources/configuration/menu.json",
    );
    assert_eq!(resources::configuration::MENU_JSON.1, "\"Start\"\n");

    assert_eq!(
        resources::configuration::TRANSLATIONS_CSV.0,
        "examples/resources/configuration/translations.csv",
    );
    assert_eq!(resources::configuration::TRANSLATIONS_CSV.1, "Back\n");

    assert_eq!(resources::CREDITS_MD.0, "examples/resources/credits.md");
    assert_eq!(resources::CREDITS_MD.1, "Foo Bar\n");

    assert_eq!(
        resources::world::levels::TUTORIAL_JSON.0,
        "examples/resources/world/levels/tutorial.json",
    );
    assert_eq!(resources::world::levels::TUTORIAL_JSON.1, "\"Hi\"\n");

    assert_eq!(
        resources::world::PHYSICAL_CONSTANTS_JSON.0,
        "examples/resources/world/physical_constants.json",
    );
    assert_eq!(resources::world::PHYSICAL_CONSTANTS_JSON.1, "7e-3\n");
}
