// This example is explained in the introduction of `../README.md`.

#[iftree::include_file_tree("paths = '/my_assets/**'")]
pub struct Asset {
    contents_str: &'static str,
}

pub fn main() {
    assert_eq!(
        base::my_assets::FILE_A.contents_str,
        "… contents of `file_a`\n",
    );
    assert_eq!(
        base::my_assets::FILE_B.contents_str,
        "… contents of `file_b`\n",
    );
    assert_eq!(
        base::my_assets::subfolder::FILE_C.contents_str,
        "… and of `file_c`\n",
    );
    assert_eq!(ASSETS.len(), 3);
    assert_eq!(ASSETS[0].contents_str, "… contents of `file_a`\n");
}
