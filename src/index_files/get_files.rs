use crate::model;
use std::path;
use std::vec;

pub fn main(canonical_paths: vec::Vec<path::PathBuf>) -> vec::Vec<model::File> {
    canonical_paths.into_iter().map(get_file).collect()
}

fn get_file(canonical_path: path::PathBuf) -> model::File {
    let full_path = path::PathBuf::from("resources").join(&canonical_path);
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
        let actual = main(vec![
            path::PathBuf::from("world/physical_constants.json"),
            path::PathBuf::from("configuration/menu.json"),
        ]);

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
