macro_rules! visit_array_base {
    ($length:literal, $($contents:expr)*) => {
        pub static ASSETS: [Asset; $length] = [$($contents,)*];
    };
}

macro_rules! visit_array_file {
    ($name:literal, $id:ident, $index:literal, $relative_path:literal, $absolute_path:literal) => {
        Asset {
            contents: include_str!($absolute_path),
        }
    };
}

macro_rules! visit_identifiers_base {
    ($length:literal, $($contents:item)*) => {
        visit_identifiers_folder! { "base", base, $($contents)* }
    };
}

macro_rules! visit_identifiers_folder {
    ($name:literal, $id:ident, $($contents:item)*) => {
        pub mod $id {
            use super::Asset;
            use super::ASSETS;

            $($contents)*
        }
    };
}

macro_rules! visit_identifiers_file {
    ($name:literal, $id:ident, $index:literal, $relative_path:literal, $absolute_path:literal) => {
        pub static $id: &Asset = &ASSETS[$index];
    };
}

#[iftree::include_file_tree(
    "
paths = '/my_assets/**'

[[template]]
visit_base = 'visit_array_base'
visit_file = 'visit_array_file'

[[template]]
visit_base = 'visit_identifiers_base'
visit_folder = 'visit_identifiers_folder'
visit_file = 'visit_identifiers_file'
"
)]
pub struct Asset {
    contents: &'static str,
}

pub fn main() {
    assert_eq!(ASSETS.len(), 3);
    assert_eq!(ASSETS[0].contents, "… contents `file_a`\n");
    assert_eq!(ASSETS[1].contents, "… contents `file_b`\n");
    assert_eq!(ASSETS[2].contents, "… and `file_c`\n");

    assert_eq!(base::my_assets::FILE_A.contents, "… contents `file_a`\n");
    assert_eq!(base::my_assets::FILE_B.contents, "… contents `file_b`\n");
    assert_eq!(
        base::my_assets::subfolder::FILE_C.contents,
        "… and `file_c`\n",
    );
}
