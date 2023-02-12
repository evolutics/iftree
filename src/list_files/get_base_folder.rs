use crate::model;
use std::env;
use std::path;

pub fn main(
    configuration: &model::Configuration,
    get_environment_variable: &dyn Fn(&str) -> Result<String, env::VarError>,
) -> model::Result<path::PathBuf> {
    if configuration.base_folder.is_absolute() {
        Ok(configuration.base_folder.clone())
    } else {
        let mut base_folder = get_root_folder(configuration, get_environment_variable)?;
        base_folder.push(&configuration.base_folder);
        Ok(base_folder)
    }
}

fn get_root_folder(
    configuration: &model::Configuration,
    get_environment_variable: &dyn Fn(&str) -> Result<String, env::VarError>,
) -> model::Result<path::PathBuf> {
    let name = &configuration.root_folder_variable;

    match get_environment_variable(name) {
        Err(source) => Err(model::Error::EnvironmentVariable {
            name: name.clone(),
            source,
        }),

        Ok(folder) => Ok(folder.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod given_base_folder_is_absolute {
        use super::*;
        use std::fs;

        #[test]
        fn handles() {
            let base_folder = fs::canonicalize(".").unwrap();
            assert!(base_folder.is_absolute());

            let actual = main(
                &model::Configuration {
                    base_folder: base_folder.clone(),
                    ..model::stubs::configuration()
                },
                &|_| unreachable!(),
            );

            let actual = actual.unwrap();
            let expected = base_folder;
            assert_eq!(actual, expected);
        }
    }

    mod given_base_folder_is_relative {
        use super::*;

        #[test]
        fn given_environment_variable_it_handles_concatenation() {
            let actual = main(
                &model::Configuration {
                    base_folder: "b/c".into(),
                    root_folder_variable: "ROOT_FOLDER".into(),
                    ..model::stubs::configuration()
                },
                &|name| {
                    Ok((if name == "ROOT_FOLDER" {
                        "/a"
                    } else {
                        unreachable!()
                    })
                    .into())
                },
            );

            let actual = actual.unwrap();
            let expected = path::PathBuf::from("/a/b/c");
            assert_eq!(actual, expected);
        }

        #[test]
        fn given_no_such_environment_variable_it_errs() {
            let actual = main(
                &model::Configuration {
                    base_folder: "a/b".into(),
                    root_folder_variable: "ROOT_FOLDER".into(),
                    ..model::stubs::configuration()
                },
                &|_| Err(env::VarError::NotPresent),
            );

            let actual = actual.unwrap_err();
            let expected = model::Error::EnvironmentVariable {
                name: "ROOT_FOLDER".into(),
                source: env::VarError::NotPresent,
            };
            assert_eq!(actual, expected);
        }
    }
}
