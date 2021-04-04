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
        let mut base_folder = get_environment_base_folder(configuration, get_environment_variable)?;
        base_folder.push(&configuration.base_folder);
        Ok(base_folder)
    }
}

fn get_environment_base_folder(
    configuration: &model::Configuration,
    get_environment_variable: &dyn Fn(&str) -> Result<String, env::VarError>,
) -> model::Result<path::PathBuf> {
    let name = &configuration.base_folder_environment_variable;

    match get_environment_variable(name) {
        Err(source) => Err(model::Error::EnvironmentVariable(
            model::EnvironmentVariableError {
                name: name.clone(),
                source,
            },
        )),

        Ok(folder) => Ok(path::PathBuf::from(folder)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod given_configured_base_folder_is_absolute {
        use super::*;
        use std::fs;

        #[test]
        fn gets_configured_base_folder() {
            let configured_base_folder = fs::canonicalize(".").unwrap();
            assert!(configured_base_folder.is_absolute());

            let actual = main(
                &model::Configuration {
                    base_folder: configured_base_folder.clone(),
                    ..model::stubs::configuration()
                },
                &|_| unreachable!(),
            );

            let actual = actual.unwrap();
            let expected = configured_base_folder;
            assert_eq!(actual, expected);
        }
    }

    mod given_configured_base_folder_is_relative {
        use super::*;

        #[test]
        fn given_environment_variable_it_gets_concatenation() {
            let actual = main(
                &model::Configuration {
                    base_folder: path::PathBuf::from("b/c"),
                    base_folder_environment_variable: String::from("BASE_FOLDER"),
                    ..model::stubs::configuration()
                },
                &|name| {
                    Ok(String::from(if name == "BASE_FOLDER" {
                        "/a"
                    } else {
                        unreachable!()
                    }))
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
                    base_folder: path::PathBuf::from("a/b"),
                    base_folder_environment_variable: String::from("BASE_FOLDER"),
                    ..model::stubs::configuration()
                },
                &|_| Err(env::VarError::NotPresent),
            );

            let actual = actual.unwrap_err();
            let expected = model::Error::EnvironmentVariable(model::EnvironmentVariableError {
                name: String::from("BASE_FOLDER"),
                source: env::VarError::NotPresent,
            });
            assert_eq!(actual, expected);
        }
    }
}
