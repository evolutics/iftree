use super::get_base_folder;
use super::get_files;
use super::get_paths;
use crate::model;
use std::env;
use std::vec;

pub fn main(configuration: &model::Configuration) -> model::Result<vec::Vec<model::File>> {
    let base_folder = get_base_folder::main(configuration, &|name| env::var(name))?;
    let paths = get_paths::main(configuration, &base_folder)?;
    get_files::main(base_folder, paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path;

    #[test]
    fn handles() {
        let actual = main(&model::Configuration {
            paths: String::from("/examples/assets/credits.md"),
            base_folder: path::PathBuf::new(),
            root_folder_variable: String::from("CARGO_MANIFEST_DIR"),
            ..model::stubs::configuration()
        });

        let actual = actual.unwrap();
        let expected = vec![model::File {
            relative_path: model::RelativePath::from("examples/assets/credits.md"),
            absolute_path: String::from(
                fs::canonicalize("examples/assets/credits.md")
                    .unwrap()
                    .to_str()
                    .unwrap(),
            ),
        }];
        assert_eq!(actual, expected);
    }
}
