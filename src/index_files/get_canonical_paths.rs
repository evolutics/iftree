use crate::model;
use std::path;
use std::vec;

pub fn main(full_resource_folder: &path::Path) -> model::Result<vec::Vec<path::PathBuf>> {
    iterate_entries(full_resource_folder)
        .into_iter()
        .filter_map(|entry| match entry {
            Err(error) => Some(Err(model::Error::Ignore(error))),
            Ok(entry) => match entry.metadata() {
                Err(error) => Some(Err(model::Error::Ignore(error))),
                Ok(metadata) => {
                    if metadata.is_dir() {
                        None
                    } else {
                        Some(canonicalize_path(full_resource_folder, entry))
                    }
                }
            },
        })
        .collect()
}

fn iterate_entries(full_resource_folder: &path::Path) -> ignore::Walk {
    ignore::WalkBuilder::new(full_resource_folder)
        .standard_filters(false)
        .build()
}

fn canonicalize_path(
    full_resource_folder: &path::Path,
    entry: ignore::DirEntry,
) -> model::Result<path::PathBuf> {
    match entry.path().strip_prefix(full_resource_folder) {
        Err(error) => Err(model::Error::StripPrefix(error)),
        Ok(canonical_path) => Ok(canonical_path.to_path_buf()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets() {
        let actual = main(path::Path::new("examples/resources"));

        let mut actual = actual.unwrap();
        actual.sort();
        assert_eq!(
            actual,
            vec![
                path::PathBuf::from(".env"),
                path::PathBuf::from("configuration/menu.json"),
                path::PathBuf::from("configuration/translations.csv"),
                path::PathBuf::from("credits.md"),
                path::PathBuf::from("world/levels/tutorial.json"),
                path::PathBuf::from("world/physical_constants.json"),
            ],
        );
    }
}
