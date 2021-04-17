use once_cell::sync;

macro_rules! is_read_only {
    ($relative_path:literal, $absolute_path:literal) => {
        once_cell::sync::Lazy::new(|| {
            std::path::Path::new($absolute_path)
                .metadata()
                .map(|metadata| metadata.permissions().readonly())
                .ok()
        })
    };
}

#[iftree::include_file_tree(
    "
paths = '/examples/resources/credits.md'

[field_templates]
is_read_only = 'is_read_only!'
"
)]
pub struct Resource {
    is_read_only: sync::Lazy<Option<bool>>,
}

pub fn main() {
    use base::examples::resources;

    assert_eq!(*resources::CREDITS_MD.is_read_only, Some(false));
}
