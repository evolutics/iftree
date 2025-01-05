use std::path;
use std::sync;

macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        Asset {
            filename: sync::LazyLock::new(|| {
                path::Path::new($relative_path)
                    .file_name()
                    .and_then(|filename| filename.to_str())
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
    filename: sync::LazyLock<Option<&'a str>>,
}

fn main() {
    use base::examples::assets;

    assert_eq!(*assets::CREDITS_MD.filename, Some("credits.md"));
}
