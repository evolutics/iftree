use crate::model;
use serde::de;
use std::collections;
use std::fmt;

pub fn main(string: &str) -> Result<model::Configuration, toml::de::Error> {
    let configuration: UserConfiguration = toml::from_str(string)?;
    Ok(configuration.into())
}

#[derive(serde::Deserialize)]
struct UserConfiguration {
    resource_paths: String,
    resolve_name_collisions: Option<bool>,
    base_folder_environment_variable: Option<String>,
    fields: Option<collections::BTreeMap<model::FieldIdentifier, model::Template>>,
}

impl<'a> de::Deserialize<'a> for model::FieldIdentifier {
    fn deserialize<T: de::Deserializer<'a>>(
        deserializer: T,
    ) -> Result<model::FieldIdentifier, T::Error> {
        deserializer.deserialize_str(FieldIdentifierVisitor)
    }
}

struct FieldIdentifierVisitor;

impl<'a> de::Visitor<'a> for FieldIdentifierVisitor {
    type Value = model::FieldIdentifier;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a field identifier")
    }

    fn visit_str<T: de::Error>(self, string: &str) -> Result<Self::Value, T> {
        Ok(if string == "_" {
            model::FieldIdentifier::Anonymous
        } else {
            model::FieldIdentifier::Named(String::from(string))
        })
    }
}

impl From<UserConfiguration> for model::Configuration {
    fn from(configuration: UserConfiguration) -> Self {
        let mut fields = configuration.fields.unwrap_or_default();
        extend_fields_with_defaults(&mut fields);

        model::Configuration {
            resource_paths: configuration.resource_paths,
            resolve_name_collisions: configuration.resolve_name_collisions.unwrap_or(false),
            base_folder_environment_variable: configuration
                .base_folder_environment_variable
                .unwrap_or_else(|| String::from("CARGO_MANIFEST_DIR")),
            fields,
        }
    }
}

fn extend_fields_with_defaults(
    fields: &mut collections::BTreeMap<model::FieldIdentifier, model::Template>,
) {
    fields
        .entry(model::FieldIdentifier::Anonymous)
        .or_insert_with(|| String::from("include_str!({{absolute_path}})"));
    fields
        .entry(model::FieldIdentifier::Named(String::from("content")))
        .or_insert_with(|| String::from("include_str!({{absolute_path}})"));
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
            fields: vec![
                (
                    model::FieldIdentifier::Anonymous,
                    String::from("include_str!({{absolute_path}})"),
                ),
                (
                    model::FieldIdentifier::Named(String::from("content")),
                    String::from("include_str!({{absolute_path}})"),
                ),
            ]
            .into_iter()
            .collect(),
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

[fields]
_ = 'my::include!({{absolute_path}})'
custom = 'my::custom_include!({{absolute_path}})'
",
        );

        let actual = actual.unwrap();
        let expected = model::Configuration {
            resource_paths: String::from("my/resources/**"),
            resolve_name_collisions: true,
            base_folder_environment_variable: String::from("MY_BASE_FOLDER"),
            fields: vec![
                (
                    model::FieldIdentifier::Anonymous,
                    String::from("my::include!({{absolute_path}})"),
                ),
                (
                    model::FieldIdentifier::Named(String::from("content")),
                    String::from("include_str!({{absolute_path}})"),
                ),
                (
                    model::FieldIdentifier::Named(String::from("custom")),
                    String::from("my::custom_include!({{absolute_path}})"),
                ),
            ]
            .into_iter()
            .collect(),
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
