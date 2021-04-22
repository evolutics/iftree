use once_cell::sync;

macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        Asset {
            filename: once_cell::sync::Lazy::new(|| {
                std::path::Path::new($relative_path)
                    .file_name()
                    .and_then(|filename| filename.to_str())
            }),
        }
    };
}

#[iftree::include_file_tree(
    "
paths = '/examples/assets/credits.md'
initializer = 'initialize'
"
)]
pub struct Asset<'a> {
    filename: sync::Lazy<Option<&'a str>>,
}

pub fn main() {
    use base::examples::assets;

    assert_eq!(*assets::CREDITS_MD.filename, Some("credits.md"));
}
