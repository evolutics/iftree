use crate::model;
use toml::de;

pub fn main(string: &str) -> Result<model::Configuration, de::Error> {
    let configuration: UserConfiguration = toml::from_str(string)?;
    Ok(configuration.into())
}

#[derive(serde::Deserialize)]
struct UserConfiguration {
    resource_paths: String,
    resolve_name_collisions: Option<bool>,
    base_folder_environment_variable: Option<String>,
}

impl From<UserConfiguration> for model::Configuration {
    fn from(configuration: UserConfiguration) -> Self {
        model::Configuration {
            resource_paths: configuration.resource_paths,
            resolve_name_collisions: configuration.resolve_name_collisions.unwrap_or(false),
            base_folder_environment_variable: configuration
                .base_folder_environment_variable
                .unwrap_or_else(|| String::from("CARGO_MANIFEST_DIR")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_configuration_with_required_fields_only_using_defaults() {
        let actual = main("resource_paths = 'resources/**'");

        let actual = actual.unwrap();
        let expected = model::Configuration {
            resource_paths: String::from("resources/**"),
            resolve_name_collisions: false,
            base_folder_environment_variable: String::from("CARGO_MANIFEST_DIR"),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_valid_configuration_with_optional_fields() {
        let actual = main(
            "
resource_paths = 'my/resources/**'
resolve_name_collisions = true
base_folder_environment_variable = 'MY_BASE_FOLDER'
",
        );

        let actual = actual.unwrap();
        let expected = model::Configuration {
            resource_paths: String::from("my/resources/**"),
            resolve_name_collisions: true,
            base_folder_environment_variable: String::from("MY_BASE_FOLDER"),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_invalid_configuration() {
        let actual = main("resource_paths = #");

        let actual = actual.is_err();
        assert!(actual);
    }
}