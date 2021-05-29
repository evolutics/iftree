#[iftree::include_file_tree(
    "
paths = '**'
base_folder = 'tests/unicode_files'
template.identifiers = false
"
)]
pub struct Asset {
    relative_path: &'static str,
    contents_str: &'static str,
}

#[test]
fn main() {
    assert_eq!(ASSETS.len(), 1);

    assert_eq!(ASSETS[0].relative_path, "Åb_π_𝟙/README_ß_ŉ.md");
    assert_eq!(ASSETS[0].contents_str, "0 1##2$3±4√5👽6.7\n");
}
