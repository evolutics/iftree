use crate::model;
use ignore::overrides;
use std::path;
use std::vec;

pub fn main(
    configuration: &model::Configuration,
    full_resource_folder: &path::Path,
) -> model::Result<vec::Vec<path::PathBuf>> {
    iterate_entries(configuration, full_resource_folder)?
        .into_iter()
        .filter_map(|entry| match entry {
            Err(error) => Some(Err(model::Error::from(error))),
            Ok(entry) => match entry.metadata() {
                Err(error) => Some(Err(model::Error::from(error))),
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

fn iterate_entries(
    configuration: &model::Configuration,
    full_resource_folder: &path::Path,
) -> model::Result<ignore::Walk> {
    let filter = get_filter(configuration, full_resource_folder)?;
    Ok(ignore::WalkBuilder::new(full_resource_folder)
        .standard_filters(false)
        .overrides(filter)
        .build())
}

fn get_filter(
    configuration: &model::Configuration,
    full_resource_folder: &path::Path,
) -> model::Result<overrides::Override> {
    let mut builder = overrides::OverrideBuilder::new(full_resource_folder);
    for pattern in configuration.path_filter.lines() {
        builder.add(pattern)?;
    }
    Ok(builder.build()?)
}

fn canonicalize_path(
    full_resource_folder: &path::Path,
    entry: ignore::DirEntry,
) -> model::Result<path::PathBuf> {
    let canonical_path = entry.path().strip_prefix(full_resource_folder)?;
    Ok(canonical_path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_with_empty_filter() {
        let actual = main(
            &model::Configuration {
                path_filter: String::new(),
                ..model::stubs::configuration()
            },
            path::Path::new("examples/resources"),
        );

        let mut actual = actual.unwrap();
        actual.sort();
        let expected = vec![
            path::PathBuf::from(".env"),
            path::PathBuf::from("configuration/menu.json"),
            path::PathBuf::from("configuration/translations.csv"),
            path::PathBuf::from("credits.md"),
            path::PathBuf::from("world/levels/tutorial.json"),
            path::PathBuf::from("world/physical_constants.json"),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn gets_files_based_on_path_filter() {
        let actual = main(
            &model::Configuration {
                path_filter: String::from(
                    "*.json
!/world/levels/",
                ),
                ..model::stubs::configuration()
            },
            path::Path::new("examples/resources"),
        );

        let mut actual = actual.unwrap();
        actual.sort();
        let expected = vec![
            path::PathBuf::from("configuration/menu.json"),
            path::PathBuf::from("world/physical_constants.json"),
        ];
        assert_eq!(actual, expected);
    }
}
