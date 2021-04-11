use std::borrow;

#[iftree::include_file_tree("resource_paths = '/examples/resources/credits.md'")]
pub struct Resource {
    content: &'static str,
    get_content: fn() -> borrow::Cow<'static, str>,
    get_raw_content: fn() -> borrow::Cow<'static, [u8]>,
    raw_content: &'static [u8],
    relative_path: &'static str,
}

pub fn main() {
    use base::examples::resources;

    assert_eq!(resources::CREDITS_MD.content, "Foo Bar\n");

    assert_eq!((resources::CREDITS_MD.get_content)(), "Foo Bar\n");

    assert_eq!(
        (resources::CREDITS_MD.get_raw_content)(),
        "Foo Bar\n".as_bytes(),
    );

    assert_eq!(resources::CREDITS_MD.raw_content, "Foo Bar\n".as_bytes());

    assert_eq!(
        resources::CREDITS_MD.relative_path,
        "examples/resources/credits.md",
    );
}
