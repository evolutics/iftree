use crate::model;
use std::path;
use std::vec;

pub fn main(
    base_folder: &path::Path,
    paths: vec::Vec<path::PathBuf>,
) -> model::Result<vec::Vec<model::File>> {
    let mut files = paths
        .into_iter()
        .map(|path| get_file(base_folder, path))
        .collect::<model::Result<vec::Vec<_>>>()?;

    files.sort_unstable_by(|left, right| left.relative_path.cmp(&right.relative_path));

    Ok(files)
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
            path::Path::new("/a/b"),
            vec![
                path::PathBuf::from("/a/b/C"),
                path::PathBuf::from("/a/b/a/b"),
            ],
        );

        let actual = actual.unwrap();
        let expected = vec![
            model::File {
                relative_path: model::RelativePath::from("C"),
                absolute_path: path::PathBuf::from("/a/b/C"),
            },
            model::File {
                relative_path: model::RelativePath::from("a/b"),
                absolute_path: path::PathBuf::from("/a/b/a/b"),
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn orders_by_relative_path() {
        let actual = main(
            path::Path::new("/"),
            vec![
                path::PathBuf::from("/B"),
                path::PathBuf::from("/A"),
                path::PathBuf::from("/a"),
                path::PathBuf::from("/a/b"),
                path::PathBuf::from("/a.c"),
            ],
        );

        let actual = actual.unwrap();
        let expected = vec![
            model::File {
                relative_path: model::RelativePath::from("A"),
                absolute_path: path::PathBuf::from("/A"),
            },
            model::File {
                relative_path: model::RelativePath::from("B"),
                absolute_path: path::PathBuf::from("/B"),
            },
            model::File {
                relative_path: model::RelativePath::from("a"),
                absolute_path: path::PathBuf::from("/a"),
            },
            model::File {
                relative_path: model::RelativePath::from("a.c"),
                absolute_path: path::PathBuf::from("/a.c"),
            },
            model::File {
                relative_path: model::RelativePath::from("a/b"),
                absolute_path: path::PathBuf::from("/a/b"),
            },
        ];
        assert_eq!(actual, expected);
    }
}
