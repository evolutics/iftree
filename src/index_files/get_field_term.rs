use super::render_field_template;
use crate::model;

pub fn main(
    configuration: &model::Configuration,
    context: &render_field_template::Context,
    identifier: model::FieldIdentifier,
) -> model::Result<proc_macro2::TokenStream> {
    match configuration.field_templates.get(&identifier) {
        None => Err(model::Error::MissingFieldTemplate(identifier)),
        Some(template) => render_field_template::main(template, context),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_missing_field_template_it_errs() {
        let actual = main(
            &model::Configuration {
                field_templates: Default::default(),
                ..model::stubs::configuration()
            },
            &render_field_template::stubs::context(),
            model::FieldIdentifier::Anonymous,
        );

        let actual = actual.unwrap_err();
        let expected = model::Error::MissingFieldTemplate(model::FieldIdentifier::Anonymous);
        assert_eq!(actual, expected);
    }

    #[test]
    fn gets() {
        let actual = main(
            &model::Configuration {
                field_templates: vec![(
                    model::FieldIdentifier::Anonymous,
                    String::from("include_str!({{absolute_path}})"),
                )]
                .into_iter()
                .collect(),
                ..model::stubs::configuration()
            },
            &render_field_template::Context {
                absolute_path: "/credits.md",
                ..render_field_template::stubs::context()
            },
            model::FieldIdentifier::Anonymous,
        );

        let actual = actual.unwrap().to_string();
        let expected = quote::quote! {
            include_str!("/credits.md")
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
