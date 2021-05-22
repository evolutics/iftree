macro_rules! visit_array_base {
    ($length:literal, $($contents:expr)*) => {
        static ASSETS: [Asset; $length] = [$($contents,)*];
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
        visit_identifiers_folder! { "", base, $($contents)* }
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
paths = '/examples/assets/**'

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

fn main() {
    use base::examples::assets;

    assert_eq!(ASSETS.len(), 6);
    assert_eq!(ASSETS[0].contents, "BASE=https://example.com\n");
    assert_eq!(ASSETS[1].contents, "\"Start\"\n");
    assert_eq!(ASSETS[2].contents, "Hi {{name}}\n");
    assert_eq!(ASSETS[3].contents, "Boo Far\n");
    assert_eq!(ASSETS[4].contents, "\"Welcome\"\n");
    assert_eq!(ASSETS[5].contents, "7e-3\n");

    assert_eq!(assets::_ENV.contents, "BASE=https://example.com\n");
    assert_eq!(assets::configuration::MENU_JSON.contents, "\"Start\"\n");
    assert_eq!(
        assets::configuration::TRANSLATIONS_CSV.contents,
        "Hi {{name}}\n",
    );
    assert_eq!(assets::CREDITS_MD.contents, "Boo Far\n");
    assert_eq!(
        assets::world::levels::TUTORIAL_JSON.contents,
        "\"Welcome\"\n",
    );
    assert_eq!(assets::world::PHYSICAL_CONSTANTS_JSON.contents, "7e-3\n");
}
