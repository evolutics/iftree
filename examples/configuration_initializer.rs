macro_rules! my_initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        Asset {
            path_length: $relative_path.len(),
            relative_path: $relative_path,
            get_text_content: {
                fn get() -> Option<String> {
                    std::fs::read_to_string($absolute_path).ok()
                }
                get
            },
        }
    };
}

#[iftree::include_file_tree(
    "
paths = '/examples/assets/credits.md'
initializer = 'my_initialize'
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
