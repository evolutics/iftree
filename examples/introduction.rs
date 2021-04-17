// Introductory example of `README.md`.

#[iftree::include_file_tree("paths = '/my_resources/**'")]
pub struct Resource {
    content: &'static str,
}

pub fn main() {
    assert_eq!(
        base::my_resources::FILE_A.content,
        "… contents of `file_a`\n",
    );
    assert_eq!(
        base::my_resources::FILE_B.content,
        "… contents of `file_b`\n",
    );
    assert_eq!(
        base::my_resources::subfolder::FILE_C.content,
        "… contents of `file_c`\n",
    );
}
