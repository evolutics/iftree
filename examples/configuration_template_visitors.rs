macro_rules! visit_array_base {
    ($length:literal, $($contents:expr)*) => {
        pub static ASSETS: [Asset; $length] = [$($contents,)*];
    };
}

macro_rules! visit_array_file {
    ($identifier:ident, $index:literal, $relative_path:literal, $absolute_path:literal) => {
        Asset {
            contents_str: include_str!($absolute_path),
        }
    };
}

macro_rules! visit_identifiers_base {
    ($length:literal, $($contents:item)*) => {
        visit_identifiers_folder! { base, "base", $($contents)* }
    };
}

macro_rules! visit_identifiers_folder {
    ($identifier:ident, $name:literal, $($contents:item)*) => {
        pub mod $identifier {
            use super::Asset;
            use super::ASSETS;

            $($contents)*
        }
    };
}

macro_rules! visit_identifiers_file {
    ($identifier:ident, $index:literal, $relative_path:literal, $absolute_path:literal) => {
        pub static $identifier: &Asset = &ASSETS[$index];
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
    contents_str: &'static str,
}

pub fn main() {
    assert_eq!(ASSETS.len(), 3);
    assert_eq!(ASSETS[0].contents_str, "… contents `file_a`\n");
    assert_eq!(ASSETS[1].contents_str, "… contents `file_b`\n");
    assert_eq!(ASSETS[2].contents_str, "… and `file_c`\n");

    assert_eq!(
        base::my_assets::FILE_A.contents_str,
        "… contents `file_a`\n",
    );
    assert_eq!(
        base::my_assets::FILE_B.contents_str,
        "… contents `file_b`\n",
    );
    assert_eq!(
        base::my_assets::subfolder::FILE_C.contents_str,
        "… and `file_c`\n",
    );
}
