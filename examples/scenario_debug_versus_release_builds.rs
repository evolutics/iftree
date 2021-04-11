macro_rules! load {
    ($relative_path:literal, $absolute_path:literal) => {
        if cfg!(debug_assertions) {
            concat!("Debug: ", include_str!($absolute_path))
        } else {
            concat!("Release: ", include_str!($absolute_path))
        }
    };
}

#[iftree::include_file_tree(
    "
resource_paths = '/examples/resources/credits.md'

[field_templates]
data = 'load!'
"
)]
pub struct Resource {
    data: &'static str,
}

pub fn main() {
    use base::examples::resources;

    if cfg!(debug_assertions) {
        assert_eq!(resources::CREDITS_MD.data, "Debug: Foo Bar\n");
    } else {
        assert_eq!(resources::CREDITS_MD.data, "Release: Foo Bar\n");
    }
}
