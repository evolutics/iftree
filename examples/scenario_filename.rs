use once_cell::sync;

macro_rules! filename {
    ($relative_path:literal, $absolute_path:literal) => {
        once_cell::sync::Lazy::new(|| {
            std::path::Path::new($relative_path)
                .file_name()
                .and_then(|filename| filename.to_str())
        })
    };
}

#[iftree::include_file_tree(
    "
resource_paths = 'examples/resources/credits.md'

[field_templates]
filename = 'filename!'
"
)]
pub struct Resource<'a> {
    filename: sync::Lazy<Option<&'a str>>,
}

pub fn main() {
    use base::examples::resources;

    assert_eq!(*resources::CREDITS_MD.filename, Some("credits.md"));
}
