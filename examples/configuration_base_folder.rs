#[files_embedded_as_modules::embed_files_as_modules(
    "
resource_paths = 'levels/**'
base_folder = 'examples/resources/world'
"
)]
pub struct Resource {
    relative_path: &'static str,
}

pub fn main() {
    assert_eq!(
        base::levels::TUTORIAL_JSON.relative_path,
        "levels/tutorial.json",
    );
}
