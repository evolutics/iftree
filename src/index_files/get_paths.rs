use crate::model;
use ignore::overrides;
use std::path;
use std::vec;

pub fn main(
    configuration: &model::Configuration,
    base_folder: &path::Path,
) -> model::Result<vec::Vec<path::PathBuf>> {
    iterate_entries(configuration, base_folder)?
        .into_iter()
        .filter_map(|entry| match entry {
            Err(error) => Some(Err(model::Error::from(error))),
            Ok(entry) => match entry.metadata() {
                Err(error) => Some(Err(model::Error::from(error))),
                Ok(metadata) => {
                    if metadata.is_dir() {
                        None
                    } else {
                        Some(relativize_path(base_folder, entry))
                    }
                }
            },
        })
        .collect()
}

fn iterate_entries(
    configuration: &model::Configuration,
    base_folder: &path::Path,
) -> model::Result<ignore::Walk> {
    let filter = get_filter(configuration, base_folder)?;
    Ok(ignore::WalkBuilder::new(base_folder)
        .standard_filters(false)
        .overrides(filter)
        .build())
}

fn get_filter(
    configuration: &model::Configuration,
    base_folder: &path::Path,
) -> model::Result<overrides::Override> {
    let mut builder = overrides::OverrideBuilder::new(base_folder);
    for pattern in configuration.resource_paths.lines() {
        builder.add(pattern)?;
    }
    Ok(builder.build()?)
}

fn relativize_path(
    base_folder: &path::Path,
    entry: ignore::DirEntry,
) -> model::Result<path::PathBuf> {
    let relative_path = entry.path().strip_prefix(base_folder)?;
    Ok(relative_path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_from_single_resource_path() {
        let actual = main(
            &model::Configuration {
                resource_paths: String::from("/examples/resources/**"),
                ..model::stubs::configuration()
            },
            path::Path::new("."),
        );

        let mut actual = actual.unwrap();
        actual.sort();
        let expected = vec![
            path::PathBuf::from("examples/resources/.env"),
            path::PathBuf::from("examples/resources/configuration/menu.json"),
            path::PathBuf::from("examples/resources/configuration/translations.csv"),
            path::PathBuf::from("examples/resources/credits.md"),
            path::PathBuf::from("examples/resources/world/levels/tutorial.json"),
            path::PathBuf::from("examples/resources/world/physical_constants.json"),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn gets_from_multiple_resource_paths() {
        let actual = main(
            &model::Configuration {
                resource_paths: String::from(
                    "/examples/resources/configuration/**
/examples/resources/world/**",
                ),
                ..model::stubs::configuration()
            },
            path::Path::new("."),
        );

        let mut actual = actual.unwrap();
        actual.sort();
        let expected = vec![
            path::PathBuf::from("examples/resources/configuration/menu.json"),
            path::PathBuf::from("examples/resources/configuration/translations.csv"),
            path::PathBuf::from("examples/resources/world/levels/tutorial.json"),
            path::PathBuf::from("examples/resources/world/physical_constants.json"),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn gets_from_include_and_exclude_resource_paths() {
        let actual = main(
            &model::Configuration {
                resource_paths: String::from(
                    "/examples/resources/**/*.json
!/examples/resources/world/levels/",
                ),
                ..model::stubs::configuration()
            },
            path::Path::new("."),
        );

        let mut actual = actual.unwrap();
        actual.sort();
        let expected = vec![
            path::PathBuf::from("examples/resources/configuration/menu.json"),
            path::PathBuf::from("examples/resources/world/physical_constants.json"),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn gets_without_hidden_files() {
        let actual = main(
            &model::Configuration {
                resource_paths: String::from(
                    "/examples/resources/*
!.*",
                ),
                ..model::stubs::configuration()
            },
            path::Path::new("."),
        );

        let mut actual = actual.unwrap();
        actual.sort();
        let expected = vec![path::PathBuf::from("examples/resources/credits.md")];
        assert_eq!(actual, expected);
    }
}
