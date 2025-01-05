use std::sync;

macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        Asset {
            media_type: sync::LazyLock::new(|| {
                let media_type = mime_guess::from_path($relative_path).first_or_octet_stream();
                media_type.essence_str().into()
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
pub struct Asset {
    media_type: sync::LazyLock<String>,
}

fn main() {
    use base::examples::assets;

    assert_eq!(*assets::CREDITS_MD.media_type, "text/markdown");
}
