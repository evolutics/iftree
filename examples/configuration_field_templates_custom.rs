macro_rules! string_length {
    ($relative_path:literal, $absolute_path:literal) => {
        $relative_path.len()
    };
}

#[files_embedded_as_modules::embed_files_as_modules(
    "
resource_paths = 'examples/resources/credits.md'

[field_templates]
path_length = 'string_length!'
"
)]
pub struct Resource {
    path_length: usize,
}

pub fn main() {
    use base::examples::resources;

    assert_eq!(resources::CREDITS_MD.path_length, 29);
}
