use once_cell::sync;
use std::path;

macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        Asset {
            extension: sync::Lazy::new(|| {
                path::Path::new($relative_path)
                    .extension()
                    .and_then(|extension| extension.to_str())
            }),
        }
    };
}

#[iftree::include_file_tree(
    "
paths = '/examples/assets/**'
template.initializer = 'initialize'
"
)]
pub struct Asset<'a> {
    extension: sync::Lazy<Option<&'a str>>,
}

pub fn main() {
    use base::examples::assets;

    assert_eq!(*assets::CREDITS_MD.extension, Some("md"));
}
