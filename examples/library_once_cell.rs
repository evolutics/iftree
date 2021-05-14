use once_cell::sync;

macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        Asset {
            first_word: sync::Lazy::new(|| {
                include_str!($absolute_path)
                    .split_whitespace()
                    .next()
                    .map(String::from)
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
    first_word: sync::Lazy<Option<String>>,
}

fn main() {
    use base::examples::assets;

    assert_eq!(*assets::CREDITS_MD.first_word, Some(String::from("Boo")));
}
