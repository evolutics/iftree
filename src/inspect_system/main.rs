use super::get_base_folder;
use super::get_paths;
use crate::model;
use std::env;

pub fn main(configuration: &model::Configuration) -> model::Result<model::SystemData> {
    let base_folder = get_base_folder::main(configuration, &|name| env::var(name))?;
    let paths = get_paths::main(configuration, &base_folder)?;
    Ok(model::SystemData { base_folder, paths })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path;

    #[test]
    fn handles() {
        let actual = main(&model::Configuration {
            paths: String::from("/examples/resources/credits.md"),
            base_folder: path::PathBuf::new(),
            root_folder_variable: String::from("CARGO_MANIFEST_DIR"),
            ..model::stubs::configuration()
        });

        let actual = actual.unwrap();
        let base_folder = fs::canonicalize(".").unwrap();
        let paths = vec![base_folder.join("examples/resources/credits.md")];
        let expected = model::SystemData { base_folder, paths };
        assert_eq!(actual, expected);
    }
}
