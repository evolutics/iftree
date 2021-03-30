use crate::model;
use std::env;
use std::path;

pub fn main(
    get_environment_variable: &dyn Fn(&str) -> Result<String, env::VarError>,
) -> model::Result<path::PathBuf> {
    let override_name = "FEAM_BASE_FOLDER";

    let folder = match get_environment_variable(override_name) {
        Err(env::VarError::NotPresent) => {
            let default_name = "CARGO_MANIFEST_DIR";

            get_environment_variable(default_name).map_err(|source| {
                model::Error::EnvironmentVariable(model::EnvironmentVariableError {
                    name: String::from(default_name),
                    source,
                    appendix: Some(format!(
                        "Try setting the base for relative paths \
                        via the environment variable {:?}.",
                        override_name,
                    )),
                })
            })
        }

        other => other.map_err(|source| {
            model::Error::EnvironmentVariable(model::EnvironmentVariableError {
                name: String::from(override_name),
                source,
                appendix: None,
            })
        }),
    }?;

    Ok(path::PathBuf::from(folder))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_feam_base_folder_it_gets_it() {
        let actual = main(&|name| {
            Ok(String::from(if name == "FEAM_BASE_FOLDER" {
                "/a"
            } else {
                "/b"
            }))
        });

        let actual = actual.unwrap();
        let expected = path::PathBuf::from("/a");
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_no_feam_base_folder_but_cargo_manifest_dir_it_gets_it() {
        let actual = main(&|name| {
            if name == "CARGO_MANIFEST_DIR" {
                Ok(String::from("/a"))
            } else {
                Err(env::VarError::NotPresent)
            }
        });

        let actual = actual.unwrap();
        let expected = path::PathBuf::from("/a");
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_neither_environment_variable_it_errs() {
        let actual = main(&|_| Err(env::VarError::NotPresent));

        let actual = match actual {
            Err(model::Error::EnvironmentVariable(error)) => error,
            _ => unreachable!(),
        };
        let expected = model::EnvironmentVariableError {
            name: String::from("CARGO_MANIFEST_DIR"),
            source: env::VarError::NotPresent,
            appendix: Some(String::from(
                "Try setting the base for relative paths \
                via the environment variable \"FEAM_BASE_FOLDER\".",
            )),
        };
        assert_eq!(actual, expected);
    }
}
