use super::print_field_term;
use crate::model;

pub fn main(
    resource_type: &model::ResourceType<model::Template>,
    file: &model::File,
) -> proc_macro2::TokenStream {
    let type_identifier = &resource_type.identifier;

    let context = print_field_term::Context {
        relative_path: &file.relative_path.0,
        absolute_path: &file.absolute_path.to_string_lossy(),
    };

    match &resource_type.structure {
        model::ResourceStructure::Unit => quote::quote! {
            #type_identifier
        },

        model::ResourceStructure::TypeAlias(template) => {
            print_field_term::main(&template, &context)
        }

        model::ResourceStructure::NamedFields(named_templates) => {
            let content: proc_macro2::TokenStream = named_templates
                .iter()
                .map(|(name, template)| {
                    let name = quote::format_ident!("{}", name);
                    let term = print_field_term::main(template, &context);
                    quote::quote! { #name: #term, }
                })
                .collect();

            quote::quote! {
                #type_identifier {
                    #content
                }
            }
        }

        model::ResourceStructure::TupleFields(templates) => {
            let content: proc_macro2::TokenStream = templates
                .iter()
                .map(|template| {
                    let term = print_field_term::main(template, &context);
                    quote::quote! { #term, }
                })
                .collect();

            quote::quote! {
                #type_identifier(
                    #content
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path;

    #[test]
    fn gets_template_context() {
        let actual = main(
            &model::ResourceType {
                identifier: quote::format_ident!("Resource"),
                structure: model::ResourceStructure::TupleFields(vec![
                    model::Template::RelativePath,
                    model::Template::Content,
                ]),
            },
            &model::File {
                relative_path: model::RelativePath::from("b"),
                absolute_path: path::PathBuf::from("/a/b"),
            },
        );

        let actual = actual.to_string();
        let expected = quote::quote! {
            Resource(
                "b",
                include_str!("/a/b"),
            )
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[cfg(test)]
    mod resource_cases {
        use super::*;

        #[test]
        fn prints_unit() {
            let actual = main(
                &model::ResourceType {
                    identifier: quote::format_ident!("MyUnit"),
                    structure: model::ResourceStructure::Unit,
                },
                &model::stubs::file(),
            );

            let actual = actual.to_string();
            let expected = quote::quote! { MyUnit }.to_string();
            assert_eq!(actual, expected);
        }

        #[test]
        fn prints_type_alias() {
            let actual = main(
                &model::ResourceType {
                    structure: model::ResourceStructure::TypeAlias(model::Template::Content),
                    ..model::stubs::resource_type()
                },
                &model::File {
                    absolute_path: path::PathBuf::from("/a/b"),
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
        fn prints_named_fields() {
            let actual = main(
                &model::ResourceType {
                    identifier: quote::format_ident!("MyNamedFields"),
                    structure: model::ResourceStructure::NamedFields(vec![(
                        String::from("raw_content"),
                        model::Template::RawContent,
                    )]),
                },
                &model::File {
                    absolute_path: path::PathBuf::from("/a/b"),
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
        fn prints_tuple_fields() {
            let actual = main(
                &model::ResourceType {
                    identifier: quote::format_ident!("MyTupleFields"),
                    structure: model::ResourceStructure::TupleFields(vec![
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
