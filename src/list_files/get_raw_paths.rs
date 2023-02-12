use crate::model;
use ignore::overrides;
use std::path;

pub fn main(
    configuration: &model::Configuration,
    base_folder: &path::Path,
) -> model::Result<Vec<path::PathBuf>> {
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
                        Some(Ok(entry.into_path()))
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
    for pattern in configuration.paths.lines() {
        builder.add(pattern)?;
    }
    let filter = builder.build()?;

    if filter.is_empty() {
        ignore_everything(base_folder)
    } else {
        Ok(filter)
    }
}

fn ignore_everything(base_folder: &path::Path) -> model::Result<overrides::Override> {
    Ok(overrides::OverrideBuilder::new(base_folder)
        .add("!*")?
        .build()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_empty_paths() {
        let actual = main(
            &model::Configuration {
                paths: String::new(),
                ..model::stubs::configuration()
            },
            path::Path::new("."),
        );

        let actual = actual.unwrap();
        assert!(actual.is_empty());
    }

    #[test]
    fn handles_single_path() {
        let actual = main(
            &model::Configuration {
                paths: "/examples/assets/**".into(),
                ..model::stubs::configuration()
            },
            path::Path::new("."),
        );

        let mut actual = actual.unwrap();
        actual.sort();
        let expected = Vec::<path::PathBuf>::from([
            "./examples/assets/.env".into(),
            "./examples/assets/configuration/menu.json".into(),
            "./examples/assets/configuration/translations.csv".into(),
            "./examples/assets/credits.md".into(),
            "./examples/assets/world/levels/tutorial.json".into(),
            "./examples/assets/world/physical_constants.json".into(),
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_multiple_paths() {
        let actual = main(
            &model::Configuration {
                paths: "/examples/assets/configuration/**
/examples/assets/world/**"
                    .into(),
                ..model::stubs::configuration()
            },
            path::Path::new("."),
        );

        let mut actual = actual.unwrap();
        actual.sort();
        let expected = Vec::<path::PathBuf>::from([
            "./examples/assets/configuration/menu.json".into(),
            "./examples/assets/configuration/translations.csv".into(),
            "./examples/assets/world/levels/tutorial.json".into(),
            "./examples/assets/world/physical_constants.json".into(),
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_negated_patterns() {
        let actual = main(
            &model::Configuration {
                paths: "/examples/assets/**/*.json
!/examples/assets/world/levels/"
                    .into(),
                ..model::stubs::configuration()
            },
            path::Path::new("."),
        );

        let mut actual = actual.unwrap();
        actual.sort();
        let expected = Vec::<path::PathBuf>::from([
            "./examples/assets/configuration/menu.json".into(),
            "./examples/assets/world/physical_constants.json".into(),
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_without_hidden_files() {
        let actual = main(
            &model::Configuration {
                paths: "/examples/assets/*
!.*"
                .into(),
                ..model::stubs::configuration()
            },
            path::Path::new("."),
        );

        let mut actual = actual.unwrap();
        actual.sort();
        let expected = Vec::<path::PathBuf>::from(["./examples/assets/credits.md".into()]);
        assert_eq!(actual, expected);
    }
}
