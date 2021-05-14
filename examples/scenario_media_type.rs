use once_cell::sync;

macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        Asset {
            media_type: sync::Lazy::new(|| {
                let media_type = mime_guess::from_path($relative_path).first_or_octet_stream();
                String::from(media_type.essence_str())
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
    media_type: sync::Lazy<String>,
}

fn main() {
    use base::examples::assets;

    assert_eq!(*assets::CREDITS_MD.media_type, "text/markdown");
}
