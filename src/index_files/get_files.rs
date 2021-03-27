use crate::model;
use std::path;
use std::vec;

pub fn main(base_folder: &path::Path, paths: vec::Vec<path::PathBuf>) -> vec::Vec<model::File> {
    paths
        .into_iter()
        .map(|path| get_file(base_folder, path))
        .collect()
}

fn get_file(base_folder: &path::Path, relative_path: path::PathBuf) -> model::File {
    let absolute_path = base_folder.join(&relative_path);
    model::File {
        relative_path,
        absolute_path,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets() {
        let actual = main(
            path::Path::new("/resources"),
            vec![
                path::PathBuf::from("world/physical_constants.json"),
                path::PathBuf::from("configuration/menu.json"),
            ],
        );

        let expected = vec![
            model::File {
                relative_path: path::PathBuf::from("world/physical_constants.json"),
                absolute_path: path::PathBuf::from("/resources/world/physical_constants.json"),
            },
            model::File {
                relative_path: path::PathBuf::from("configuration/menu.json"),
                absolute_path: path::PathBuf::from("/resources/configuration/menu.json"),
            },
        ];
        assert_eq!(actual, expected);
    }
}
