use crate::model;
use std::env;
use std::error;
use std::fmt;
use std::path;

impl fmt::Display for model::Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            model::Error::EnvironmentVariableCargoManifestDir(error) => {
                let name = "CARGO_MANIFEST_DIR";
                match error {
                    env::VarError::NotPresent => write!(
                        formatter,
                        "The environment variable {:?} is not defined. \
                        It is required to resolve the resource folder path. \
                        As a workaround, try defining it manually.",
                        name,
                    ),
                    env::VarError::NotUnicode(data) => {
                        write!(
                            formatter,
                            "The environment variable {:?} \
                            has invalid Unicode data: {:?}",
                            name, data,
                        )
                    }
                }
            }

            model::Error::Ignore(error) => write!(formatter, "{}", error),

            model::Error::NameCollisions(collisions) => {
                let configuration = "resolve_name_collisions = true";
                write!(
                    formatter,
                    "Name collisions in generated code; \
                    rename files or configure {:?}:",
                    configuration,
                )?;
                for collision in collisions {
                    let existing_file_hint = match &collision.existing_filename {
                        None => String::new(),
                        Some(filename) => format!("with {:?} ", filename),
                    };
                    write!(
                        formatter,
                        "\n- {:?} collides {}on identifier {:?}.",
                        collision.colliding_file.relative_path,
                        existing_file_hint,
                        collision.identifier,
                    )?;
                }
                Ok(())
            }

            model::Error::PathStripPrefix(error) => write!(formatter, "{}", error),
        }
    }
}

impl error::Error for model::Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            model::Error::EnvironmentVariableCargoManifestDir(error) => Some(error),
            model::Error::Ignore(error) => Some(error),
            model::Error::NameCollisions(_) => None,
            model::Error::PathStripPrefix(error) => Some(error),
        }
    }
}

impl From<ignore::Error> for model::Error {
    fn from(error: ignore::Error) -> Self {
        model::Error::Ignore(error)
    }
}

impl From<path::StripPrefixError> for model::Error {
    fn from(error: path::StripPrefixError) -> Self {
        model::Error::PathStripPrefix(error)
    }
}
