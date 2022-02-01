// Say you have these files:
//
//     my_assets/
//     ├── file_a
//     ├── file_b
//     └── folder/
//         └── file_c

// Include data from this file tree in your code like so:
#[iftree::include_file_tree("paths = '/my_assets/**'")]
pub struct MyAsset {
    relative_path: &'static str,
    contents_str: &'static str,
}

fn main() {
    // Based on this, an array `ASSETS` of `MyAsset` instances is generated:
    assert_eq!(ASSETS.len(), 3);
    assert_eq!(ASSETS[0].relative_path, "my_assets/file_a");
    assert_eq!(ASSETS[0].contents_str, "… contents file_a\n");
    assert_eq!(ASSETS[1].contents_str, "… contents file_b\n");
    assert_eq!(ASSETS[2].contents_str, "… file_c\n");

    // Also, variables `base::x::y::MY_FILE` are generated (named by file path):
    assert_eq!(base::my_assets::FILE_A.relative_path, "my_assets/file_a");
    assert_eq!(base::my_assets::FILE_A.contents_str, "… contents file_a\n");
    assert_eq!(base::my_assets::FILE_B.contents_str, "… contents file_b\n");
    assert_eq!(base::my_assets::folder::FILE_C.contents_str, "… file_c\n");
}
