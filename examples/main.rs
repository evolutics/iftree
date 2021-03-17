#[files_embedded_as_modules::embed_files_as_modules]
pub struct Resource {
    get: &'static str,
}

pub fn main() {
    assert_eq!(resources::credits().get, "Foo Bar\n");
}
