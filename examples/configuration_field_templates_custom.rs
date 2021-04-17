macro_rules! string_length {
    ($relative_path:literal, $absolute_path:literal) => {
        $relative_path.len()
    };
}

macro_rules! get_text_content {
    ($relative_path:literal, $absolute_path:literal) => {{
        fn get() -> Option<String> {
            std::fs::read_to_string($absolute_path).ok()
        }

        get
    }};
}

#[iftree::include_file_tree(
    "
paths = '/examples/resources/credits.md'

[field_templates]
path_length = 'string_length!'
get_text_content = 'get_text_content!'
"
)]
pub struct Resource {
    path_length: usize,
    relative_path: &'static str,
    get_text_content: fn() -> Option<String>,
}

pub fn main() {
    use base::examples::resources;

    assert_eq!(resources::CREDITS_MD.path_length, 29);

    assert_eq!(
        resources::CREDITS_MD.relative_path,
        "examples/resources/credits.md",
    );

    assert_eq!(
        (resources::CREDITS_MD.get_text_content)(),
        Some(String::from("Foo Bar\n")),
    );
}
