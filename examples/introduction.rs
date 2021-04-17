// Introductory example of `README.md`.

#[iftree::include_file_tree("paths = '/my_assets/**'")]
pub struct Asset {
    content: &'static str,
}

pub fn main() {
    assert_eq!(base::my_assets::FILE_A.content, "… contents of `file_a`\n");
    assert_eq!(base::my_assets::FILE_B.content, "… contents of `file_b`\n");
    assert_eq!(
        base::my_assets::subfolder::FILE_C.content,
        "… contents of `file_c`\n",
    );
}
