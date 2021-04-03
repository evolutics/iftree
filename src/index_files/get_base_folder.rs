use crate::model;
use std::env;
use std::path;

pub fn main(
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

    #[test]
    fn given_environment_variable_it_gets_its_value() {
        let actual = main(
            &model::Configuration {
                base_folder_environment_variable: String::from("BASE_FOLDER"),
                ..model::stubs::configuration()
            },
            &|name| {
                Ok(String::from(if name == "BASE_FOLDER" {
                    "/a"
                } else {
                    "/b"
                }))
            },
        );

        let actual = actual.unwrap();
        let expected = path::PathBuf::from("/a");
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_no_such_environment_variable_it_errs() {
        let actual = main(
            &model::Configuration {
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
