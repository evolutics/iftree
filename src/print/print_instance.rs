use super::print_template;
use crate::model;

pub fn main(type_: &model::Type<model::Template>, file: &model::File) -> proc_macro2::TokenStream {
    let type_name = &type_.name;

    let context = print_template::Context {
        relative_path: &file.relative_path.0,
        absolute_path: &file.absolute_path,
    };

    match &type_.structure {
        model::TypeStructure::Unit => quote::quote! {
            #type_name
        },

        model::TypeStructure::TypeAlias(template) => print_template::main(&template, &context),

        model::TypeStructure::NamedFields(named_templates) => {
            let content: proc_macro2::TokenStream = named_templates
                .iter()
                .map(|(name, template)| {
                    let name = quote::format_ident!("{}", name);
                    let term = print_template::main(template, &context);
                    quote::quote! { #name: #term, }
                })
                .collect();

            quote::quote! {
                #type_name {
                    #content
                }
            }
        }

        model::TypeStructure::TupleFields(templates) => {
            let content: proc_macro2::TokenStream = templates
                .iter()
                .map(|template| {
                    let term = print_template::main(template, &context);
                    quote::quote! { #term, }
                })
                .collect();

            quote::quote! {
                #type_name(
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
    fn handles_template_context() {
        let actual = main(
            &model::Type {
                name: quote::format_ident!("Asset"),
                structure: model::TypeStructure::TupleFields(vec![
                    model::Template::RelativePath,
                    model::Template::Content,
                ]),
            },
            &model::File {
                relative_path: model::RelativePath::from("b"),
                absolute_path: String::from("/a/b"),
            },
        );

        let actual = actual.to_string();
        let expected = quote::quote! {
            Asset(
                "b",
                include_str!("/a/b"),
            )
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[cfg(test)]
    mod type_cases {
        use super::*;

        #[test]
        fn handles_unit() {
            let actual = main(
                &model::Type {
                    name: quote::format_ident!("MyUnit"),
                    structure: model::TypeStructure::Unit,
                },
                &model::stubs::file(),
            );

            let actual = actual.to_string();
            let expected = quote::quote! { MyUnit }.to_string();
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_type_alias() {
            let actual = main(
                &model::Type {
                    structure: model::TypeStructure::TypeAlias(model::Template::Content),
                    ..model::stubs::type_()
                },
                &model::File {
                    absolute_path: String::from("/a/b"),
                    ..model::stubs::file()
                },
            );

            let actual = actual.to_string();
            let expected = quote::quote! {
                include_str!("/a/b")
            }
            .to_string();
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_named_fields() {
            let actual = main(
                &model::Type {
                    name: quote::format_ident!("MyNamedFields"),
                    structure: model::TypeStructure::NamedFields(vec![(
                        String::from("raw_content"),
                        model::Template::RawContent,
                    )]),
                },
                &model::File {
                    absolute_path: String::from("/a/b"),
                    ..model::stubs::file()
                },
            );

            let actual = actual.to_string();
            let expected = quote::quote! {
                MyNamedFields {
                    raw_content: include_bytes!("/a/b"),
                }
            }
            .to_string();
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_tuple_fields() {
            let actual = main(
                &model::Type {
                    name: quote::format_ident!("MyTupleFields"),
                    structure: model::TypeStructure::TupleFields(vec![
                        model::Template::RelativePath,
                    ]),
                },
                &model::File {
                    relative_path: model::RelativePath::from("b"),
                    ..model::stubs::file()
                },
            );

            let actual = actual.to_string();
            let expected = quote::quote! {
                MyTupleFields(
                    "b",
                )
            }
            .to_string();
            assert_eq!(actual, expected);
        }
    }
}
