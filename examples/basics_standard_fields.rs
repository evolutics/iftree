use std::borrow;

#[iftree::include_file_tree("paths = '/examples/assets/**'")]
pub struct Asset {
    contents_bytes: &'static [u8],
    contents_str: &'static str,
    get_bytes: fn() -> borrow::Cow<'static, [u8]>,
    get_str: fn() -> borrow::Cow<'static, str>,
    relative_path: &'static str,
}

fn main() {
    use base::examples::assets;

    assert_eq!(assets::CREDITS_MD.contents_bytes, b"Boo Far\n");

    assert_eq!(assets::CREDITS_MD.contents_str, "Boo Far\n");

    assert_eq!((assets::CREDITS_MD.get_bytes)(), &b"Boo Far\n"[..]);

    assert_eq!((assets::CREDITS_MD.get_str)(), "Boo Far\n");

    assert_eq!(
        assets::CREDITS_MD.relative_path,
        "examples/assets/credits.md",
    );
}
