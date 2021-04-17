use super::main;
use std::error;
use std::fmt;
use std::path;

impl PartialEq for main::IgnoreError {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

impl fmt::Display for main::Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            main::Error::EnvironmentVariable { name, source } => write!(
                formatter,
                "Unable to get environment variable {:?}: {}",
                name, source,
            ),

            main::Error::Ignore(main::IgnoreError(error)) => write!(formatter, "{}", error),

            main::Error::MissingFieldTemplate(field) => {
                let field = String::from(field.clone());
                write!(
                    formatter,
                    "No template for field {:?}. Add one to your configuration as follows:
```
[field_templates]
{} = …
```",
                    field, field,
                )
            }

            main::Error::NameCollision {
                identifier,
                competitors,
            } => {
                writeln!(
                    formatter,
                    "Files collide on generated identifier {:?}:",
                    identifier,
                )?;
                for competitor in competitors {
                    writeln!(formatter, "- {:?}", competitor.0)?;
                }
                write!(
                    formatter,
                    "Rename one of the files or configure {:?}.",
                    "module_tree = false",
                )
            }

            main::Error::PathStripPrefix(error) => write!(formatter, "{}", error),
        }
    }
}

impl error::Error for main::Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            main::Error::EnvironmentVariable { source, .. } => Some(source),
            main::Error::Ignore(main::IgnoreError(error)) => Some(error),
            main::Error::MissingFieldTemplate(_) => None,
            main::Error::NameCollision { .. } => None,
            main::Error::PathStripPrefix(error) => Some(error),
        }
    }
}

impl From<ignore::Error> for main::Error {
    fn from(error: ignore::Error) -> Self {
        main::Error::Ignore(main::IgnoreError(error))
    }
}

impl From<path::StripPrefixError> for main::Error {
    fn from(error: path::StripPrefixError) -> Self {
        main::Error::PathStripPrefix(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[cfg(test)]
    mod display {
        use super::*;

        #[test]
        fn handles_environment_variable() {
            let actual = main::Error::EnvironmentVariable {
                name: String::from("ABC"),
                source: env::VarError::NotPresent,
            }
            .to_string();

            let expected = "Unable to get environment variable \"ABC\": \
            environment variable not found";
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_missing_field_template() {
            let actual =
                main::Error::MissingFieldTemplate(main::FieldIdentifier::Anonymous).to_string();

            let expected = "No template for field \"_\". Add one to your configuration as follows:
```
[field_templates]
_ = …
```";
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_name_collision() {
            let actual = main::Error::NameCollision {
                identifier: String::from("b_c"),
                competitors: vec![
                    main::RelativePath::from("a/B-c"),
                    main::RelativePath::from("a/b.c"),
                ],
            }
            .to_string();

            let expected = "Files collide on generated identifier \"b_c\":
- \"a/B-c\"
- \"a/b.c\"
Rename one of the files or configure \"module_tree = false\".";
            assert_eq!(actual, expected);
        }
    }
}
