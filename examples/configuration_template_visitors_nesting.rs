macro_rules! visit_base {
    ($length:literal, $($contents:expr)*) => {
        fn list_assets() -> String {
            let root_depth = 0;
            vec![$($contents(root_depth),)*].join("")
        }
    };
}

macro_rules! visit_folder {
    ($name:literal, $id:ident, $($contents:expr)*) => {{
        fn list_folder(depth: usize) -> String {
            vec![
                indent_line(depth, &format!("{}/", $name)),
                $($contents(depth + 1),)*
            ]
            .join("")
        }
        list_folder
    }};
}

fn indent_line(depth: usize, contents: &str) -> String {
    format!("{}{}\n", "    ".repeat(depth), contents)
}

macro_rules! visit_file {
    ($name:literal, $id:ident, $index:literal, $relative_path:literal, $absolute_path:literal) => {{
        fn list_file(depth: usize) -> String {
            indent_line(depth, $name)
        }
        list_file
    }};
}

#[iftree::include_file_tree(
    "
paths = '/examples/assets/**'

[[template]]
visit_base = 'visit_base'
visit_folder = 'visit_folder'
visit_file = 'visit_file'
"
)]
pub struct Asset;

fn main() {
    assert_eq!(
        list_assets(),
        "examples/
    assets/
        .env
        configuration/
            menu.json
            translations.csv
        credits.md
        world/
            levels/
                tutorial.json
            physical_constants.json
",
    );
}
