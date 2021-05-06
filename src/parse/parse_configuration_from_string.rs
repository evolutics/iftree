use crate::model;
use std::path;
use std::vec;
use toml::de;

pub fn main(string: &str) -> Result<model::Configuration, de::Error> {
    let configuration: Configuration = toml::from_str(string)?;
    Ok(configuration.into())
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Configuration {
    paths: String,
    base_folder: Option<path::PathBuf>,
    root_folder_variable: Option<String>,
    template: Option<Template>,
    debug: Option<bool>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields, untagged)]
enum Template {
    Default {
        initializer: Option<super::path::Path>,
        identifiers: Option<bool>,
    },
    Visitors(vec::Vec<CustomVisitor>),
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct CustomVisitor {
    visit_base: Option<super::path::Path>,
    visit_folder: Option<super::path::Path>,
    visit_file: super::path::Path,
}

impl From<Configuration> for model::Configuration {
    fn from(configuration: Configuration) -> Self {
        model::Configuration {
            paths: configuration.paths,
            base_folder: configuration.base_folder.unwrap_or_else(path::PathBuf::new),
            root_folder_variable: configuration
                .root_folder_variable
                .unwrap_or_else(|| String::from("CARGO_MANIFEST_DIR")),
            template: match configuration.template {
                None => model::Template::Default {
                    initializer: None,
                    identifiers: true,
                },
                Some(template) => template.into(),
            },
            debug: configuration.debug.unwrap_or(false),
        }
    }
}

impl From<Template> for model::Template {
    fn from(template: Template) -> Self {
        match template {
            Template::Default {
                initializer,
                identifiers,
            } => model::Template::Default {
                initializer: initializer.map(|value| value.0),
                identifiers: identifiers.unwrap_or(true),
            },
            Template::Visitors(visitors) => model::Template::Visitors(
                visitors.into_iter().map(|visitor| visitor.into()).collect(),
            ),
        }
    }
}

impl From<CustomVisitor> for model::CustomVisitor {
    fn from(visitor: CustomVisitor) -> Self {
        model::CustomVisitor {
            visit_base: visitor.visit_base.map(|value| value.0),
            visit_folder: visitor.visit_folder.map(|value| value.0),
            visit_file: visitor.visit_file.0,
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
            template: model::Template::Default {
                initializer: None,
                identifiers: true,
            },
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
template.initializer = 'my_macro'
template.identifiers = false
debug = true
",
        );

        let actual = actual.unwrap();
        let expected = model::Configuration {
            paths: String::from("/my/assets/**"),
            base_folder: path::PathBuf::from("my_base"),
            root_folder_variable: String::from("MY_ROOT_FOLDER"),
            template: model::Template::Default {
                initializer: Some(syn::parse_str("my_macro").unwrap()),
                identifiers: false,
            },
            debug: true,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_valid_configuration_with_template_visitors() {
        let actual = main(
            "
paths = ''
template = [
  { visit_file = 'file' },
  { visit_base = 'my_base', visit_folder = 'my_folder', visit_file = 'my_file' },
]
",
        );

        let actual = actual.unwrap().template;
        let expected = model::Template::Visitors(vec![
            model::CustomVisitor {
                visit_base: None,
                visit_folder: None,
                visit_file: syn::parse_str("file").unwrap(),
            },
            model::CustomVisitor {
                visit_base: Some(syn::parse_str("my_base").unwrap()),
                visit_folder: Some(syn::parse_str("my_folder").unwrap()),
                visit_file: syn::parse_str("my_file").unwrap(),
            },
        ]);
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
