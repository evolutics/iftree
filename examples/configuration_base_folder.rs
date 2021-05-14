#[iftree::include_file_tree(
    "
paths = '/levels/**'
base_folder = 'examples/assets/world'
"
)]
pub struct Asset {
    relative_path: &'static str,
}

fn main() {
    assert_eq!(
        base::levels::TUTORIAL_JSON.relative_path,
        "levels/tutorial.json",
    );
}
