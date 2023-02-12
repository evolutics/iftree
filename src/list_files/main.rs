use super::get_base_folder;
use super::get_paths;
use super::get_raw_paths;
use crate::model;
use std::env;

pub fn main(configuration: &model::Configuration) -> model::Result<Vec<model::Path>> {
    let base_folder = get_base_folder::main(configuration, &|name| env::var(name))?;
    let paths = get_raw_paths::main(configuration, &base_folder)?;
    get_paths::main(base_folder, paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path;

    #[test]
    fn handles() {
        let actual = main(&model::Configuration {
            paths: "/assets/*.md".into(),
            base_folder: "examples".into(),
            root_folder_variable: "CARGO_MANIFEST_DIR".into(),
            ..model::stubs::configuration()
        });

        let actual = actual.unwrap();
        let expected = vec![model::Path {
            relative: vec!["assets".into(), "credits.md".into()],
            absolute: path::PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
                .join("examples")
                .join("assets")
                .join("credits.md")
                .into_os_string()
                .into_string()
                .unwrap(),
        }];
        assert_eq!(actual, expected);
    }
}
