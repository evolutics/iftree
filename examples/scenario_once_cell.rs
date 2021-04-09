use once_cell::sync;

macro_rules! filename_extension {
    ($relative_path:literal, $absolute_path:literal) => {
        once_cell::sync::Lazy::new(|| {
            std::path::Path::new($relative_path)
                .extension()
                .and_then(|extension| extension.to_str())
        })
    };
}

#[files_embedded_as_modules::embed_files_as_modules(
    "
resource_paths = 'examples/resources/credits.md'

[field_templates]
extension = 'filename_extension!'
"
)]
pub struct Resource<'a> {
    extension: sync::Lazy<Option<&'a str>>,
}

pub fn main() {
    use base::examples::resources;

    assert_eq!(*resources::CREDITS_MD.extension, Some("md"));
}
