use once_cell::sync;

macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        Asset {
            is_read_only: once_cell::sync::Lazy::new(|| {
                std::path::Path::new($absolute_path)
                    .metadata()
                    .map(|metadata| metadata.permissions().readonly())
                    .ok()
            }),
        }
    };
}

#[iftree::include_file_tree(
    "
paths = '/examples/assets/credits.md'
template.initializer = 'initialize'
"
)]
pub struct Asset {
    is_read_only: sync::Lazy<Option<bool>>,
}

pub fn main() {
    use base::examples::assets;

    assert_eq!(*assets::CREDITS_MD.is_read_only, Some(false));
}
