use crate::model;

pub fn main(
    resource_type: &syn::Ident,
    resource_term: &model::ResourceTerm,
) -> proc_macro2::TokenStream {
    match resource_term {
        model::ResourceTerm::Unit => quote::quote! { #resource_type },

        model::ResourceTerm::TypeAlias(term) => term.clone(),

        model::ResourceTerm::NamedFields(fields) => {
            let content: proc_macro2::TokenStream = fields
                .iter()
                .map(|(name, term)| {
                    let name = quote::format_ident!("{}", name);
                    quote::quote! { #name: #term, }
                })
                .collect();

            quote::quote! {
                #resource_type {
                    #content
                }
            }
        }

        model::ResourceTerm::TupleFields(terms) => {
            let content: proc_macro2::TokenStream =
                terms.iter().map(|term| quote::quote! { #term, }).collect();

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

    #[cfg(test)]
    mod resource_cases {
        use super::*;

        #[test]
        fn prints_unit() {
            let actual = main(
                &quote::format_ident!("Resource"),
                &model::ResourceTerm::Unit,
            );

            let actual = actual.to_string();
            let expected = quote::quote! { Resource }.to_string();
            assert_eq!(actual, expected);
        }

        #[test]
        fn prints_type_alias() {
            let actual = main(
                &quote::format_ident!("Foo"),
                &model::ResourceTerm::TypeAlias(quote::quote! {
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
                &model::ResourceTerm::NamedFields(vec![
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
                &model::ResourceTerm::TupleFields(vec![
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
}
