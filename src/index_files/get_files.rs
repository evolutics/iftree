use crate::model;
use std::path;
use std::vec;

pub fn main(
    full_resource_folder: &path::Path,
    canonical_paths: vec::Vec<path::PathBuf>,
) -> vec::Vec<model::File> {
    canonical_paths
        .into_iter()
        .map(|canonical_path| get_file(full_resource_folder, canonical_path))
        .collect()
}

fn get_file(full_resource_folder: &path::Path, canonical_path: path::PathBuf) -> model::File {
    let full_path = full_resource_folder.join(&canonical_path);
    model::File {
        canonical_path,
        full_path,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets() {
        let actual = main(
            path::Path::new("resources"),
            vec![
                path::PathBuf::from("world/physical_constants.json"),
                path::PathBuf::from("configuration/menu.json"),
            ],
        );

        assert_eq!(
            actual,
            vec![
                model::File {
                    canonical_path: path::PathBuf::from("world/physical_constants.json"),
                    full_path: path::PathBuf::from("resources/world/physical_constants.json"),
                },
                model::File {
                    canonical_path: path::PathBuf::from("configuration/menu.json"),
                    full_path: path::PathBuf::from("resources/configuration/menu.json"),
                },
            ],
        );
    }
}
