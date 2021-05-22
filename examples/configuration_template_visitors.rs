macro_rules! visit_base {
    ($length:literal, $($contents:expr)*) => {
        fn list_assets() -> String {
            vec![$($contents,)*].join("\n")
        }
    };
}

macro_rules! visit_file {
    ($name:literal, $id:ident, $index:literal, $relative_path:literal, $absolute_path:literal) => {
        $relative_path
    };
}

#[iftree::include_file_tree(
    "
paths = '/examples/assets/**'

[[template]]
visit_base = 'visit_base'
visit_file = 'visit_file'
"
)]
pub struct Asset;

fn main() {
    assert_eq!(
        list_assets(),
        "examples/assets/.env
examples/assets/configuration/menu.json
examples/assets/configuration/translations.csv
examples/assets/credits.md
examples/assets/world/levels/tutorial.json
examples/assets/world/physical_constants.json",
    );
}
