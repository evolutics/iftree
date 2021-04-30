use crate::model;
use std::path;
use std::vec;

pub fn main(
    base_folder: path::PathBuf,
    paths: vec::Vec<path::PathBuf>,
) -> model::Result<vec::Vec<model::Path>> {
    paths
        .into_iter()
        .map(|path| get_path(&base_folder, path))
        .collect()
}

fn get_path(base_folder: &path::Path, path: path::PathBuf) -> model::Result<model::Path> {
    let relative_path = model::RelativePath(get_path_string(path.strip_prefix(base_folder)?)?);
    let absolute_path = get_path_string(&path)?;

    Ok(model::Path {
        relative_path,
        absolute_path,
    })
}

fn get_path_string(path: &path::Path) -> model::Result<String> {
    match path.to_str() {
        None => Err(model::Error::PathInvalidUnicode(path.to_path_buf())),
        Some(string) => Ok(String::from(string)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles() {
        let actual = main(
            path::PathBuf::from("/a/b"),
            vec![
                path::PathBuf::from("/a/b/c"),
                path::PathBuf::from("/a/b/a/b"),
            ],
        );

        let actual = actual.unwrap();
        let expected = vec![
            model::Path {
                relative_path: model::RelativePath::from("c"),
                absolute_path: String::from("/a/b/c"),
            },
            model::Path {
                relative_path: model::RelativePath::from("a/b"),
                absolute_path: String::from("/a/b/a/b"),
            },
        ];
        assert_eq!(actual, expected);
    }
}
