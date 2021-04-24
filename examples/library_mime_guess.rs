macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        Asset {
            guess_media_type: {
                fn get() -> mime_guess::MimeGuess {
                    mime_guess::from_path($relative_path)
                }
                get
            },
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
    guess_media_type: fn() -> mime_guess::MimeGuess,
}

pub fn main() {
    use base::examples::assets;

    assert_eq!(
        (assets::CREDITS_MD.guess_media_type)(),
        mime_guess::from_ext("md"),
    );
}
