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

#[iftree::include_file_tree(
    "
paths = '/examples/assets/credits.md'

[field_templates]
extension = 'filename_extension!'
"
)]
pub struct Asset<'a> {
    extension: sync::Lazy<Option<&'a str>>,
}

pub fn main() {
    use base::examples::assets;

    assert_eq!(*assets::CREDITS_MD.extension, Some("md"));
}
