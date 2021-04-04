use std::fs;

#[files_embedded_as_modules::embed_files_as_modules(
    "
resource_paths = 'examples/resources/credits.md'
"
)]
pub struct Resource {
    absolute_path: &'static str,
    content: &'static str,
    relative_path: &'static str,
}

pub fn main() {
    use root::examples::resources;

    assert_eq!(
        resources::CREDITS_MD.absolute_path,
        fs::canonicalize("examples/resources/credits.md")
            .unwrap()
            .to_string_lossy(),
    );

    assert_eq!(resources::CREDITS_MD.content, "Foo Bar\n");

    assert_eq!(
        resources::CREDITS_MD.relative_path,
        "examples/resources/credits.md",
    );
}
