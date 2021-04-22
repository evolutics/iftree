macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        Asset {
            data: if cfg!(debug_assertions) {
                concat!("Debug: ", include_str!($absolute_path))
            } else {
                concat!("Release: ", include_str!($absolute_path))
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
    data: &'static str,
}

pub fn main() {
    use base::examples::assets;

    if cfg!(debug_assertions) {
        assert_eq!(assets::CREDITS_MD.data, "Debug: Boo Far\n");
    } else {
        assert_eq!(assets::CREDITS_MD.data, "Release: Boo Far\n");
    }
}
