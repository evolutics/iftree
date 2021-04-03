use crate::model;

pub fn main(
    absolute_path: &str,
    identifier: model::FieldIdentifier,
) -> model::Result<proc_macro2::TokenStream> {
    match &identifier {
        model::FieldIdentifier::Anonymous => Ok(quote::quote! {
            include_str!(#absolute_path)
        }),

        model::FieldIdentifier::Named(name) => match name.as_ref() {
            "content" => Ok(quote::quote! {
                include_str!(#absolute_path)
            }),
            _ => Err(model::Error::MissingImplementation(identifier)),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_anonymous() {
        let actual = main("/credits.md", model::FieldIdentifier::Anonymous);

        let actual = actual.unwrap().to_string();
        let expected = quote::quote! {
            include_str!("/credits.md")
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn gets_named() {
        let actual = main(
            "/credits.md",
            model::FieldIdentifier::Named(String::from("content")),
        );

        let actual = actual.unwrap().to_string();
        let expected = quote::quote! {
            include_str!("/credits.md")
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
