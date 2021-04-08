use crate::data;
use crate::model;
use serde::de;
use std::fmt;
use std::path;
use std::vec;

pub fn main(string: &str) -> Result<model::Configuration, toml::de::Error> {
    let configuration: UserConfiguration = toml::from_str(string)?;
    Ok(configuration.into())
}

#[derive(serde::Deserialize)]
struct UserConfiguration {
    resource_paths: String,
    base_folder: Option<path::PathBuf>,
    root_folder_variable: Option<String>,

    resolve_name_collisions: Option<bool>,
    generate_array: Option<bool>,

    field_templates: Option<model::FieldTemplates>,
}

impl<'a> de::Deserialize<'a> for model::Template {
    fn deserialize<T: de::Deserializer<'a>>(deserializer: T) -> Result<model::Template, T::Error> {
        deserializer.deserialize_string(TemplateVisitor)
    }
}

struct TemplateVisitor;

impl de::Visitor<'_> for TemplateVisitor {
    type Value = model::Template;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "one of {}, or a macro name followed by `!`",
            data::PREDEFINED_TEMPLATES
                .iter()
                .map(|(name, _)| format!("{:?}", name))
                .collect::<vec::Vec<_>>()
                .join(", "),
        )
    }

    fn visit_str<T: de::Error>(self, string: &str) -> Result<Self::Value, T> {
        match string.strip_suffix('!') {
            None => match data::PREDEFINED_TEMPLATES
                .iter()
                .find(|(name, _)| *name == string)
            {
                None => Err(de::Error::invalid_value(de::Unexpected::Str(string), &self)),
                Some((_, template)) => Ok(template.clone()),
            },

            Some(macro_name) => Ok(model::Template::Custom(String::from(macro_name))),
        }
    }
}

impl From<UserConfiguration> for model::Configuration {
    fn from(configuration: UserConfiguration) -> Self {
        let mut field_templates = configuration.field_templates.unwrap_or_default();
        extend_field_templates_with_predefined(&mut field_templates);

        model::Configuration {
            resource_paths: configuration.resource_paths,
            base_folder: configuration.base_folder.unwrap_or_else(path::PathBuf::new),
            root_folder_variable: configuration
                .root_folder_variable
                .unwrap_or_else(|| String::from("CARGO_MANIFEST_DIR")),

            resolve_name_collisions: configuration.resolve_name_collisions.unwrap_or(false),
            generate_array: configuration.generate_array.unwrap_or(false),

            field_templates,
        }
    }
}

fn extend_field_templates_with_predefined(field_templates: &mut model::FieldTemplates) {
    for (name, template) in data::PREDEFINED_TEMPLATES {
        field_templates
            .entry(model::FieldIdentifier::Named(String::from(*name)))
            .or_insert_with(|| template.clone());
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
            base_folder: path::PathBuf::new(),
            root_folder_variable: String::from("CARGO_MANIFEST_DIR"),

            resolve_name_collisions: false,
            generate_array: false,

            field_templates: vec![
                (
                    model::FieldIdentifier::Named(String::from("absolute_path")),
                    model::Template::AbsolutePath,
                ),
                (
                    model::FieldIdentifier::Named(String::from("content")),
                    model::Template::Content,
                ),
                (
                    model::FieldIdentifier::Named(String::from("raw_content")),
                    model::Template::RawContent,
                ),
                (
                    model::FieldIdentifier::Named(String::from("relative_path")),
                    model::Template::RelativePath,
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
base_folder = 'base'
root_folder_variable = 'MY_ROOT_FOLDER'

resolve_name_collisions = true
generate_array = true

[field_templates]
_ = 'my::include!'
custom = 'my::custom_include!'
3 = 'my::another_include!'
4 = 'raw_content'
",
        );

        let actual = actual.unwrap();
        let expected = model::Configuration {
            resource_paths: String::from("my/resources/**"),
            base_folder: path::PathBuf::from("base"),
            root_folder_variable: String::from("MY_ROOT_FOLDER"),

            resolve_name_collisions: true,
            generate_array: true,

            field_templates: vec![
                (
                    model::FieldIdentifier::Anonymous,
                    model::Template::Custom(String::from("my::include")),
                ),
                (
                    model::FieldIdentifier::Named(String::from("absolute_path")),
                    model::Template::AbsolutePath,
                ),
                (
                    model::FieldIdentifier::Named(String::from("content")),
                    model::Template::Content,
                ),
                (
                    model::FieldIdentifier::Named(String::from("custom")),
                    model::Template::Custom(String::from("my::custom_include")),
                ),
                (
                    model::FieldIdentifier::Named(String::from("raw_content")),
                    model::Template::RawContent,
                ),
                (
                    model::FieldIdentifier::Named(String::from("relative_path")),
                    model::Template::RelativePath,
                ),
                (
                    model::FieldIdentifier::Indexed(3),
                    model::Template::Custom(String::from("my::another_include")),
                ),
                (
                    model::FieldIdentifier::Indexed(4),
                    model::Template::RawContent,
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
