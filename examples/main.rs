#[files_embedded_as_modules::embed_files_as_modules]
pub struct Resource {
    get: &'static str,
}

pub fn main() {
    assert_eq!(resources::configuration::MENU_JSON.get, "\"Start\"\n");
    assert_eq!(resources::configuration::TRANSLATIONS_CSV.get, "Back\n");
    assert_eq!(resources::CREDITS_MD.get, "Foo Bar\n");
    assert_eq!(resources::world::levels::TUTORIAL_JSON.get, "\"Hi\"\n");
    assert_eq!(resources::world::PHYSICAL_CONSTANTS_JSON.get, "7e-3\n");
}
