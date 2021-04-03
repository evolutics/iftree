use crate::model;

pub fn main(
    configuration: &model::Configuration,
    absolute_path: &str,
    identifier: model::FieldIdentifier,
) -> model::Result<proc_macro2::TokenStream> {
    match configuration.field_templates.get(&identifier) {
        None => Err(model::Error::MissingImplementation(identifier)),

        Some(template) => match template.as_ref() {
            "include_str!({{absolute_path}})" => Ok(quote::quote! {
                include_str!(#absolute_path)
            }),
            _ => Err(model::Error::NonStandardTemplate(template.clone())),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_missing_implementation_it_errs() {
        let actual = main(
            &model::Configuration {
                field_templates: Default::default(),
                ..model::stubs::configuration()
            },
            "/credits.md",
            model::FieldIdentifier::Anonymous,
        );

        let actual = actual.unwrap_err();
        let expected = model::Error::MissingImplementation(model::FieldIdentifier::Anonymous);
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
            "/credits.md",
            model::FieldIdentifier::Anonymous,
        );

        let actual = actual.unwrap().to_string();
        let expected = quote::quote! {
            include_str!("/credits.md")
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_non_standard_template_it_errs() {
        let actual = main(
            &model::Configuration {
                field_templates: vec![(
                    model::FieldIdentifier::Anonymous,
                    String::from("my_include!({{absolute_path}})"),
                )]
                .into_iter()
                .collect(),
                ..model::stubs::configuration()
            },
            "/credits.md",
            model::FieldIdentifier::Anonymous,
        );

        let actual = actual.unwrap_err();
        let expected =
            model::Error::NonStandardTemplate(String::from("my_include!({{absolute_path}})"));
        assert_eq!(actual, expected);
    }
}
