use crate::model;
use std::path;

pub fn main(
    base_folder: path::PathBuf,
    paths: Vec<path::PathBuf>,
) -> model::Result<Vec<model::Path>> {
    paths
        .into_iter()
        .map(|path| get_path(&base_folder, path))
        .collect()
}

fn get_path(base_folder: &path::Path, path: path::PathBuf) -> model::Result<model::Path> {
    let relative = get_path_components(path.strip_prefix(base_folder)?)?;
    let absolute = get_path_string(&path)?;

    Ok(model::Path { relative, absolute })
}

fn get_path_components(path: &path::Path) -> model::Result<Vec<String>> {
    path.iter()
        .map(|component| match component.to_str() {
            None => Err(model::Error::PathInvalidUnicode(path.to_path_buf())),
            Some(string) => Ok(string.into()),
        })
        .collect()
}

fn get_path_string(path: &path::Path) -> model::Result<String> {
    match path.to_str() {
        None => Err(model::Error::PathInvalidUnicode(path.to_path_buf())),
        Some(string) => Ok(string.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles() {
        let actual = main("/a/b".into(), vec!["/a/b/c".into(), "/a/b/a/b".into()]);

        let actual = actual.unwrap();
        let expected = vec![
            model::Path {
                relative: vec!["c".into()],
                absolute: "/a/b/c".into(),
            },
            model::Path {
                relative: vec!["a".into(), "b".into()],
                absolute: "/a/b/a/b".into(),
            },
        ];
        assert_eq!(actual, expected);
    }
}
