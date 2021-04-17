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
paths = '/examples/assets/credits.md'

[field_templates]
filename = 'filename!'
"
)]
pub struct Asset<'a> {
    filename: sync::Lazy<Option<&'a str>>,
}

pub fn main() {
    use base::examples::assets;

    assert_eq!(*assets::CREDITS_MD.filename, Some("credits.md"));
}
