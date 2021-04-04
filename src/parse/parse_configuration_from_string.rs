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
    generate_array: Option<bool>,
    base_folder_environment_variable: Option<String>,
    field_templates: Option<collections::BTreeMap<model::FieldIdentifier, model::Template>>,
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
            match string.parse() {
                Err(_) => model::FieldIdentifier::Named(String::from(string)),
                Ok(index) => model::FieldIdentifier::Indexed(index),
            }
        })
    }
}

impl From<UserConfiguration> for model::Configuration {
    fn from(configuration: UserConfiguration) -> Self {
        let mut field_templates = configuration.field_templates.unwrap_or_default();
        extend_field_templates_with_defaults(&mut field_templates);

        model::Configuration {
            resource_paths: configuration.resource_paths,
            resolve_name_collisions: configuration.resolve_name_collisions.unwrap_or(false),
            generate_array: configuration.generate_array.unwrap_or(true),
            base_folder_environment_variable: configuration
                .base_folder_environment_variable
                .unwrap_or_else(|| String::from("CARGO_MANIFEST_DIR")),
            field_templates,
        }
    }
}

fn extend_field_templates_with_defaults(
    field_templates: &mut collections::BTreeMap<model::FieldIdentifier, model::Template>,
) {
    field_templates
        .entry(model::FieldIdentifier::Anonymous)
        .or_insert_with(|| String::from("include_str!({{absolute_path}})"));
    field_templates
        .entry(model::FieldIdentifier::Named(String::from("absolute_path")))
        .or_insert_with(|| String::from("{{absolute_path}}"));
    field_templates
        .entry(model::FieldIdentifier::Named(String::from("content")))
        .or_insert_with(|| String::from("include_str!({{absolute_path}})"));
    field_templates
        .entry(model::FieldIdentifier::Named(String::from("relative_path")))
        .or_insert_with(|| String::from("{{relative_path}}"));
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
            generate_array: true,
            base_folder_environment_variable: String::from("CARGO_MANIFEST_DIR"),
            field_templates: vec![
                (
                    model::FieldIdentifier::Anonymous,
                    String::from("include_str!({{absolute_path}})"),
                ),
                (
                    model::FieldIdentifier::Named(String::from("absolute_path")),
                    String::from("{{absolute_path}}"),
                ),
                (
                    model::FieldIdentifier::Named(String::from("content")),
                    String::from("include_str!({{absolute_path}})"),
                ),
                (
                    model::FieldIdentifier::Named(String::from("relative_path")),
                    String::from("{{relative_path}}"),
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
generate_array = false
base_folder_environment_variable = 'MY_BASE_FOLDER'

[field_templates]
_ = 'my::include!({{absolute_path}})'
custom = 'my::custom_include!({{absolute_path}})'
3 = 'my::another_include!({{absolute_path}})'
",
        );

        let actual = actual.unwrap();
        let expected = model::Configuration {
            resource_paths: String::from("my/resources/**"),
            resolve_name_collisions: true,
            generate_array: false,
            base_folder_environment_variable: String::from("MY_BASE_FOLDER"),
            field_templates: vec![
                (
                    model::FieldIdentifier::Anonymous,
                    String::from("my::include!({{absolute_path}})"),
                ),
                (
                    model::FieldIdentifier::Named(String::from("absolute_path")),
                    String::from("{{absolute_path}}"),
                ),
                (
                    model::FieldIdentifier::Named(String::from("content")),
                    String::from("include_str!({{absolute_path}})"),
                ),
                (
                    model::FieldIdentifier::Named(String::from("custom")),
                    String::from("my::custom_include!({{absolute_path}})"),
                ),
                (
                    model::FieldIdentifier::Named(String::from("relative_path")),
                    String::from("{{relative_path}}"),
                ),
                (
                    model::FieldIdentifier::Indexed(3),
                    String::from("my::another_include!({{absolute_path}})"),
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
