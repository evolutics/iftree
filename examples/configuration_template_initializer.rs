macro_rules! my_initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        Asset {
            path_length: $relative_path.len(),

            relative_path: $relative_path,

            get_first_word: {
                fn get() -> Option<&'static str> {
                    include_str!($absolute_path).split_whitespace().next()
                }
                get
            },

            version: if cfg!(debug_assertions) {
                "debug"
            } else {
                "release"
            },
        }
    };
}

#[iftree::include_file_tree(
    "
paths = '/examples/assets/credits.md'
template.initializer = 'my_initialize'
"
)]
pub struct Asset {
    path_length: usize,
    relative_path: &'static str,
    get_first_word: fn() -> Option<&'static str>,
    version: &'static str,
}

pub fn main() {
    use base::examples::assets;

    assert_eq!(assets::CREDITS_MD.path_length, 26);

    assert_eq!(
        assets::CREDITS_MD.relative_path,
        "examples/assets/credits.md",
    );

    assert_eq!((assets::CREDITS_MD.get_first_word)(), Some("Boo"));

    if cfg!(debug_assertions) {
        assert_eq!(assets::CREDITS_MD.version, "debug");
    } else {
        assert_eq!(assets::CREDITS_MD.version, "release");
    }
}
