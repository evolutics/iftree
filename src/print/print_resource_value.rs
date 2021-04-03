use crate::model;

pub fn main(
    resource_type: &syn::Ident,
    fields: &model::Fields<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    match fields {
        model::Fields::TypeAlias(value) => value.clone(),

        model::Fields::NamedFields(fields) => {
            let content: proc_macro2::TokenStream = fields
                .iter()
                .map(|(name, value)| {
                    let name = quote::format_ident!("{}", name);
                    quote::quote! { #name: #value, }
                })
                .collect();

            quote::quote! {
                #resource_type {
                    #content
                }
            }
        }

        model::Fields::TupleFields(fields) => {
            let content: proc_macro2::TokenStream = fields
                .iter()
                .map(|value| quote::quote! { #value, })
                .collect();

            quote::quote! {
                #resource_type(
                    #content
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_type_alias() {
        let actual = main(
            &quote::format_ident!("Foo"),
            &model::Fields::TypeAlias(quote::quote! {
                include_str!("/credits.md")
            }),
        );

        let actual = actual.to_string();
        let expected = quote::quote! {
            include_str!("/credits.md")
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn prints_named_fields() {
        let actual = main(
            &quote::format_ident!("Resource"),
            &model::Fields::NamedFields(vec![
                (
                    String::from("content"),
                    quote::quote! { include_str!("/credits.md") },
                ),
                (
                    String::from("media_type"),
                    quote::quote! { "text/markdown" },
                ),
            ]),
        );

        let actual = actual.to_string();
        let expected = quote::quote! {
            Resource {
                content: include_str!("/credits.md"),
                media_type: "text/markdown",
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn prints_tuple_fields() {
        let actual = main(
            &quote::format_ident!("Resource"),
            &model::Fields::TupleFields(vec![
                quote::quote! { include_str!("/credits.md") },
                quote::quote! { "text/markdown" },
            ]),
        );

        let actual = actual.to_string();
        let expected = quote::quote! {
            Resource(
                include_str!("/credits.md"),
                "text/markdown",
            )
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
