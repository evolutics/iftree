use super::print_template;
use crate::model;

pub fn main(view: &model::View, file: &model::File) -> proc_macro2::TokenStream {
    match &view.initializer {
        model::Initializer::Default(templates) => print_default(&view.type_, templates, file),
        model::Initializer::Macro(name) => print_macro(name, file),
    }
}

fn print_default(
    type_: &syn::Ident,
    templates: &model::TypeStructure<model::Template>,
    file: &model::File,
) -> proc_macro2::TokenStream {
    let context = print_template::Context {
        relative_path: &file.relative_path.0,
        absolute_path: &file.absolute_path,
    };

    match templates {
        model::TypeStructure::Unit => quote::quote! { #type_ },

        model::TypeStructure::TypeAlias(template) => print_template::main(template, &context),

        model::TypeStructure::NamedFields(field_templates) => {
            let contents: proc_macro2::TokenStream = field_templates
                .iter()
                .map(|(field, template)| {
                    let field = quote::format_ident!("{}", field);
                    let term = print_template::main(template, &context);
                    quote::quote! { #field: #term, }
                })
                .collect();

            quote::quote! { #type_ { #contents } }
        }

        model::TypeStructure::TupleFields(templates) => {
            let contents: proc_macro2::TokenStream = templates
                .iter()
                .map(|template| {
                    let term = print_template::main(template, &context);
                    quote::quote! { #term, }
                })
                .collect();

            quote::quote! { #type_(#contents) }
        }
    }
}

fn print_macro(macro_: &str, file: &model::File) -> proc_macro2::TokenStream {
    let macro_name = quote::format_ident!("{}", macro_);
    let relative_path = &file.relative_path.0;
    let absolute_path = &file.absolute_path;

    quote::quote! { #macro_name!(#relative_path, #absolute_path) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod default {
        use super::*;

        #[test]
        fn handles_template_context() {
            let actual = main(
                &model::View {
                    type_: quote::format_ident!("Asset"),
                    initializer: model::Initializer::Default(model::TypeStructure::TupleFields(
                        vec![model::Template::RelativePath, model::Template::Content],
                    )),
                    ..model::stubs::view()
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
                    &model::View {
                        type_: quote::format_ident!("MyUnit"),
                        initializer: model::Initializer::Default(model::TypeStructure::Unit),
                        ..model::stubs::view()
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
                    &model::View {
                        initializer: model::Initializer::Default(model::TypeStructure::TypeAlias(
                            model::Template::Content,
                        )),
                        ..model::stubs::view()
                    },
                    &model::File {
                        absolute_path: String::from("/a/b"),
                        ..model::stubs::file()
                    },
                );

                let actual = actual.to_string();
                let expected = quote::quote! { include_str!("/a/b") }.to_string();
                assert_eq!(actual, expected);
            }

            #[test]
            fn handles_named_fields() {
                let actual = main(
                    &model::View {
                        type_: quote::format_ident!("MyNamedFields"),
                        initializer: model::Initializer::Default(
                            model::TypeStructure::NamedFields(vec![(
                                String::from("raw_content"),
                                model::Template::RawContent,
                            )]),
                        ),
                        ..model::stubs::view()
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
                    &model::View {
                        type_: quote::format_ident!("MyTupleFields"),
                        initializer: model::Initializer::Default(
                            model::TypeStructure::TupleFields(vec![model::Template::RelativePath]),
                        ),
                        ..model::stubs::view()
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

    #[test]
    fn macro_() {
        let actual = main(
            &model::View {
                initializer: model::Initializer::Macro(String::from("abc")),
                ..model::stubs::view()
            },
            &model::File {
                relative_path: model::RelativePath::from("b"),
                absolute_path: String::from("/a/b"),
            },
        );

        let actual = actual.to_string();
        let expected = quote::quote! { abc!("b", "/a/b") }.to_string();
        assert_eq!(actual, expected);
    }
}
