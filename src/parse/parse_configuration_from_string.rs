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
#[serde(deny_unknown_fields)]
struct UserConfiguration {
    paths: String,
    base_folder: Option<path::PathBuf>,
    root_folder_variable: Option<String>,

    module_tree: Option<bool>,

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
            data::PREDEFINED_TEMPLATES_ORDERED
                .iter()
                .map(|(name, _)| format!("{:?}", name))
                .collect::<vec::Vec<_>>()
                .join(", "),
        )
    }

    fn visit_str<T: de::Error>(self, string: &str) -> Result<Self::Value, T> {
        match string.strip_suffix('!') {
            None => match data::PREDEFINED_TEMPLATES_ORDERED
                .binary_search_by_key(&string, |entry| entry.0)
            {
                Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(string), &self)),
                Ok(index) => Ok(data::PREDEFINED_TEMPLATES_ORDERED[index].1.clone()),
            },

            Some(macro_) => Ok(model::Template::Custom {
                macro_: String::from(macro_),
            }),
        }
    }
}

impl From<UserConfiguration> for model::Configuration {
    fn from(configuration: UserConfiguration) -> Self {
        model::Configuration {
            paths: configuration.paths,
            base_folder: configuration.base_folder.unwrap_or_else(path::PathBuf::new),
            root_folder_variable: configuration
                .root_folder_variable
                .unwrap_or_else(|| String::from("CARGO_MANIFEST_DIR")),

            module_tree: configuration.module_tree.unwrap_or(true),

            field_templates: configuration.field_templates.unwrap_or_default(),
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

            module_tree: true,

            field_templates: model::FieldTemplates::new(),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_valid_configuration_with_optional_fields() {
        let actual = main(
            "
paths = '/my/resources/**'
base_folder = 'base'
root_folder_variable = 'MY_ROOT_FOLDER'

module_tree = false

[field_templates]
_ = 'my_include!'
custom = 'custom::include!'
3 = 'raw_content'
",
        );

        let actual = actual.unwrap();
        let expected = model::Configuration {
            paths: String::from("/my/resources/**"),
            base_folder: path::PathBuf::from("base"),
            root_folder_variable: String::from("MY_ROOT_FOLDER"),

            module_tree: false,

            field_templates: vec![
                (
                    model::FieldIdentifier::Anonymous,
                    model::Template::Custom {
                        macro_: String::from("my_include"),
                    },
                ),
                (
                    model::FieldIdentifier::Named(String::from("custom")),
                    model::Template::Custom {
                        macro_: String::from("custom::include"),
                    },
                ),
                (
                    model::FieldIdentifier::Indexed(3),
                    model::Template::RawContent,
                ),
            ]
            .into_iter()
            .collect(),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_invalid_configuration_it_errs() {
        let actual = main("paths = #");

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
