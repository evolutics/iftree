use crate::model;
use std::path;
use std::vec;

pub fn main(
    base_folder: &path::Path,
    paths: vec::Vec<path::PathBuf>,
) -> model::Result<vec::Vec<model::File>> {
    paths
        .into_iter()
        .map(|path| get_file(base_folder, path))
        .collect()
}

fn get_file(base_folder: &path::Path, absolute_path: path::PathBuf) -> model::Result<model::File> {
    let relative_path = model::RelativePath(String::from(
        absolute_path.strip_prefix(base_folder)?.to_string_lossy(),
    ));

    Ok(model::File {
        relative_path,
        absolute_path,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets() {
        let actual = main(
            path::Path::new("/resources"),
            vec![
                path::PathBuf::from("/resources/world/physical_constants.json"),
                path::PathBuf::from("/resources/configuration/menu.json"),
            ],
        );

        let actual = actual.unwrap();
        let expected = vec![
            model::File {
                relative_path: model::RelativePath::from("world/physical_constants.json"),
                absolute_path: path::PathBuf::from("/resources/world/physical_constants.json"),
            },
            model::File {
                relative_path: model::RelativePath::from("configuration/menu.json"),
                absolute_path: path::PathBuf::from("/resources/configuration/menu.json"),
            },
        ];
        assert_eq!(actual, expected);
    }
}
