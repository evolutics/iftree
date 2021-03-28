use super::main;
use std::env;
use std::error;
use std::fmt;
use std::path;

impl fmt::Display for main::Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            main::Error::EnvironmentVariableCargoManifestDir(error) => {
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

            main::Error::Ignore(error) => write!(formatter, "{}", error),

            main::Error::NameCollisions(collisions) => {
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

            main::Error::PathStripPrefix(error) => write!(formatter, "{}", error),
        }
    }
}

impl error::Error for main::Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            main::Error::EnvironmentVariableCargoManifestDir(error) => Some(error),
            main::Error::Ignore(error) => Some(error),
            main::Error::NameCollisions(_) => None,
            main::Error::PathStripPrefix(error) => Some(error),
        }
    }
}

impl From<ignore::Error> for main::Error {
    fn from(error: ignore::Error) -> Self {
        main::Error::Ignore(error)
    }
}

impl From<path::StripPrefixError> for main::Error {
    fn from(error: path::StripPrefixError) -> Self {
        main::Error::PathStripPrefix(error)
    }
}
