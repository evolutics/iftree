use once_cell::sync;
use std::path;

macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        Asset {
            filename: sync::Lazy::new(|| {
                path::Path::new($relative_path)
                    .file_name()
                    .and_then(|filename| filename.to_str())
            }),
        }
    };
}

#[iftree::include_file_tree(
    "
paths = '/examples/assets/credits.md'
template.initializer = 'initialize'
"
)]
pub struct Asset<'a> {
    filename: sync::Lazy<Option<&'a str>>,
}

pub fn main() {
    use base::examples::assets;

    assert_eq!(*assets::CREDITS_MD.filename, Some("credits.md"));
}
