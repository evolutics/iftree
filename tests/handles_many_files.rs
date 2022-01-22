#[iftree::include_file_tree(
    "
paths = '**'
base_folder = 'tests/many_files'
"
)]
pub struct Asset {
    relative_path: &'static str,
    contents_str: &'static str,
}

#[test]
fn main() {
    assert_eq!(ASSETS.len(), 576);

    assert_eq!(ASSETS[3].relative_path, "100");
    assert_eq!(ASSETS[575].contents_str, "g/f/e/d/c/b/a/0\n");

    for asset in &ASSETS {
        let path = asset.relative_path;
        assert_eq!(format!("{path}\n"), asset.contents_str);
    }

    assert_eq!(base::_127.relative_path, "127");
    assert_eq!(base::e::c::b::_1.contents_str, "e/c/b/1\n");
    assert_eq!(
        base::g::f::e::d::c::b::a::_0.relative_path,
        "g/f/e/d/c/b/a/0",
    );
}
