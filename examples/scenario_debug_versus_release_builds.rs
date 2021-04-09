#[cfg(debug_assertions)]
macro_rules! load {
    ($relative_path:literal, $absolute_path:literal) => {
        concat!("Debug: ", include_str!($absolute_path))
    };
}

#[cfg(not(debug_assertions))]
macro_rules! load {
    ($relative_path:literal, $absolute_path:literal) => {
        concat!("Release: ", include_str!($absolute_path))
    };
}

#[iftree::include_file_tree(
    "
resource_paths = 'examples/resources/credits.md'

[field_templates]
data = 'load!'
"
)]
pub struct Resource {
    data: &'static str,
}

pub fn main() {
    use base::examples::resources;

    assert!(
        (resources::CREDITS_MD.data == "Debug: Foo Bar\n")
            || (resources::CREDITS_MD.data == "Release: Foo Bar\n")
    );
}
