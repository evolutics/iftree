use once_cell::sync;

macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        Asset {
            media_type_guess: once_cell::sync::Lazy::new(|| mime_guess::from_path($relative_path)),
        }
    };
}

#[iftree::include_file_tree(
    "
paths = '/examples/assets/credits.md'
initializer = 'initialize'
"
)]
pub struct Asset {
    media_type_guess: sync::Lazy<mime_guess::MimeGuess>,
}

pub fn main() {
    use base::examples::assets;

    assert_eq!(
        *assets::CREDITS_MD.media_type_guess,
        mime_guess::from_ext("md"),
    );
}
