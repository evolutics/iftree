use super::identifier;
use crate::model;
use std::path;
use toml::de;

pub fn main(string: &str) -> Result<model::Configuration, de::Error> {
    let configuration: UserConfiguration = toml::from_str(string)?;
    Ok(configuration.into())
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct UserConfiguration {
    paths: String,
    base_folder: Option<path::PathBuf>,
    root_folder_variable: Option<String>,
    initializer: Option<identifier::Identifier>,
    identifiers: Option<bool>,
    debug: Option<bool>,
}

impl From<UserConfiguration> for model::Configuration {
    fn from(configuration: UserConfiguration) -> Self {
        model::Configuration {
            paths: configuration.paths,
            base_folder: configuration.base_folder.unwrap_or_else(path::PathBuf::new),
            root_folder_variable: configuration
                .root_folder_variable
                .unwrap_or_else(|| String::from("CARGO_MANIFEST_DIR")),
            initializer: configuration.initializer.map(|identifier| identifier.0),
            identifiers: configuration.identifiers.unwrap_or(true),
            debug: configuration.debug.unwrap_or(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_valid_configuration_with_required_fields_only_using_defaults() {
        let actual = main("paths = '/a/b/**'");

        let actual = actual.unwrap();
        let expected = model::Configuration {
            paths: String::from("/a/b/**"),
            base_folder: path::PathBuf::new(),
            root_folder_variable: String::from("CARGO_MANIFEST_DIR"),
            initializer: None,
            identifiers: true,
            debug: false,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_valid_configuration_with_optional_fields() {
        let actual = main(
            "
paths = '/my/assets/**'
base_folder = 'my_base'
root_folder_variable = 'MY_ROOT_FOLDER'
initializer = 'my_macro'
identifiers = false
debug = true
",
        );

        let actual = actual.unwrap();
        let expected = model::Configuration {
            paths: String::from("/my/assets/**"),
            base_folder: path::PathBuf::from("my_base"),
            root_folder_variable: String::from("MY_ROOT_FOLDER"),
            initializer: Some(quote::format_ident!("my_macro")),
            identifiers: false,
            debug: true,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_ill_formed_configuration_it_errs() {
        let actual = main("paths = #");

        let actual = actual.is_err();
        assert!(actual);
    }

    #[test]
    fn given_required_field_is_missing_it_errs() {
        let actual = main("");

        let actual = actual.is_err();
        assert!(actual);
    }

    #[test]
    fn given_unknown_field_it_errs() {
        let actual = main(
            "
paths = 'abc'
unknown = ''
",
        );

        let actual = actual.is_err();
        assert!(actual);
    }
}
