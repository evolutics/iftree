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
paths = '/examples/assets/credits.md'

[field_templates]
path_length = 'string_length!'
get_text_content = 'get_text_content!'
"
)]
pub struct Asset {
    path_length: usize,
    relative_path: &'static str,
    get_text_content: fn() -> Option<String>,
}

pub fn main() {
    use base::examples::assets;

    assert_eq!(assets::CREDITS_MD.path_length, 26);

    assert_eq!(
        assets::CREDITS_MD.relative_path,
        "examples/assets/credits.md",
    );

    assert_eq!(
        (assets::CREDITS_MD.get_text_content)(),
        Some(String::from("Boo Far\n")),
    );
}
