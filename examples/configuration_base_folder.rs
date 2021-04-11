#[iftree::include_file_tree(
    "
resource_paths = '/levels/**'
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
