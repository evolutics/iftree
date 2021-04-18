use once_cell::sync;

macro_rules! best_media_type_guess {
    ($relative_path:literal, $absolute_path:literal) => {
        once_cell::sync::Lazy::new(|| {
            let media_type = mime_guess::from_path($relative_path).first_or_octet_stream();
            String::from(media_type.essence_str())
        })
    };
}

#[iftree::include_file_tree(
    "
paths = '/examples/assets/credits.md'

[field_templates]
media_type = 'best_media_type_guess!'
"
)]
pub struct Asset {
    media_type: sync::Lazy<String>,
}

pub fn main() {
    use base::examples::assets;

    assert_eq!(*assets::CREDITS_MD.media_type, "text/markdown");
}
