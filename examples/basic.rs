// Say you have these files:
//
//     my_assets/
//     ├── file_a
//     ├── file_b
//     └── folder/
//         └── file_c

// To include this file tree in your code, define a custom type like so:
#[iftree::include_file_tree("paths = '/my_assets/**'")]
pub struct MyAsset {
    contents_str: &'static str,
}
// Above we configure a path pattern to filter the files in `my_assets/` and its
// subfolders. For each selected file, an instance of `MyAsset` is initialized.
// The standard field `contents_str` is automatically populated with a call to
// `include_str!`, but you can plug in your own initializer.

fn main() {
    // Based on this, an array `ASSETS` with the desired file data is generated:
    assert_eq!(ASSETS.len(), 3);
    assert_eq!(ASSETS[0].contents_str, "… contents file_a\n");
    assert_eq!(ASSETS[1].contents_str, "… contents file_b\n");
    assert_eq!(ASSETS[2].contents_str, "… file_c\n");

    // Also, variables `base::x::y::MY_FILE` are generated (named by file path):
    assert_eq!(base::my_assets::FILE_A.contents_str, "… contents file_a\n");
    assert_eq!(base::my_assets::FILE_B.contents_str, "… contents file_b\n");
    assert_eq!(base::my_assets::folder::FILE_C.contents_str, "… file_c\n");
}
