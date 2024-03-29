use super::main;
use std::error;
use std::fmt;
use std::path;

impl PartialEq for main::IgnoreError {
    fn eq(&self, other: &Self) -> bool {
        format!("{self:?}") == format!("{other:?}")
    }
}

impl fmt::Display for main::Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            main::Error::EnvironmentVariable { name, source } => write!(
                formatter,
                "Unable to get environment variable {name:?}: {source}",
            ),

            main::Error::Ignore(main::IgnoreError(error)) => write!(formatter, "{error}"),

            main::Error::NoInitializer => formatter.write_str(
                "No initializer. \
                Configure one with \"template.initializer = 'a_macro'\" or \
                use standard fields to generate a default initializer.",
            ),

            main::Error::NonstandardField {
                field,
                standard_fields,
            } => {
                let field = field.to_string();
                let standard_fields = standard_fields
                    .iter()
                    .map(|field| {
                        let field = field.to_string();
                        format!("{field:?}")
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(
                    formatter,
                    "Default initializer cannot be generated \
                    as field {field:?} is not standard. \
                    Configure an initializer with \"template.initializer = 'a_macro'\" or \
                    use standard fields only ({standard_fields}).",
                )
            }

            main::Error::PathInvalidUnicode(path) => write!(
                formatter,
                "Path is not valid Unicode, consider renaming it: {path:?}",
            ),

            main::Error::PathStripPrefix(error) => write!(formatter, "{error}"),

            main::Error::UnexpectedEmptyRelativePath { absolute_path } => write!(
                formatter,
                "Unexpected empty relative path for absolute path \
                (consider reporting this): {absolute_path:?}",
            ),

            main::Error::UnexpectedPathCollision(path) => write!(
                formatter,
                "Unexpected path collision (consider reporting this): {path:?}",
            ),
        }
    }
}

impl error::Error for main::Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            main::Error::EnvironmentVariable { source, .. } => Some(source),
            main::Error::Ignore(main::IgnoreError(error)) => Some(error),
            main::Error::NoInitializer => None,
            main::Error::NonstandardField { .. } => None,
            main::Error::PathInvalidUnicode(_) => None,
            main::Error::PathStripPrefix(error) => Some(error),
            main::Error::UnexpectedEmptyRelativePath { .. } => None,
            main::Error::UnexpectedPathCollision(_) => None,
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
                name: "ABC".into(),
                source: env::VarError::NotPresent,
            }
            .to_string();

            let expected = "Unable to get environment variable \"ABC\": \
            environment variable not found";
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_ignore() {
            let actual = main::Error::Ignore(main::IgnoreError(ignore::Error::Glob {
                glob: Some("[".into()),
                err: "abc".into(),
            }))
            .to_string();

            let expected = "error parsing glob '[': abc";
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_no_initializer() {
            let actual = main::Error::NoInitializer.to_string();

            let expected = "No initializer. \
Configure one with \"template.initializer = 'a_macro'\" or \
use standard fields to generate a default initializer.";
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_nonstandard_field() {
            let actual = main::Error::NonstandardField {
                field: quote::format_ident!("abc"),
                standard_fields: vec![quote::format_ident!("xy"), quote::format_ident!("z")],
            }
            .to_string();

            let expected = "Default initializer cannot be generated \
as field \"abc\" is not standard. \
Configure an initializer with \"template.initializer = 'a_macro'\" or \
use standard fields only (\"xy\", \"z\").";
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_path_invalid_unicode() {
            let actual = main::Error::PathInvalidUnicode("a/b".into()).to_string();

            let expected = "Path is not valid Unicode, consider renaming it: \"a/b\"";
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_path_strip_prefix() {
            let actual =
                main::Error::PathStripPrefix(path::Path::new("ab").strip_prefix("bc").unwrap_err())
                    .to_string();

            let expected = "prefix not found";
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_unexpected_empty_relative_path() {
            let actual = main::Error::UnexpectedEmptyRelativePath {
                absolute_path: "/a".into(),
            }
            .to_string();

            let expected = "Unexpected empty relative path for absolute path \
(consider reporting this): \"/a\"";
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_unexpected_path_collision() {
            let actual = main::Error::UnexpectedPathCollision("a/b".into()).to_string();

            let expected = "Unexpected path collision (consider reporting this): \"a/b\"";
            assert_eq!(actual, expected);
        }
    }
}
