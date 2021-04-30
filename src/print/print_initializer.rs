use super::print_populator;
use crate::model;

pub fn main(view: &model::View, path: &model::Path) -> proc_macro2::TokenStream {
    match &view.initializer {
        model::Initializer::Default(populators) => print_default(&view.type_, populators, path),
        model::Initializer::Macro(name) => print_macro(name, path),
    }
}

fn print_default(
    type_: &syn::Ident,
    populators: &model::TypeStructure<model::Populator>,
    path: &model::Path,
) -> proc_macro2::TokenStream {
    let context = print_populator::Context {
        relative_path: &path.relative.0,
        absolute_path: &path.absolute,
    };

    match populators {
        model::TypeStructure::Unit => quote::quote! { #type_ },

        model::TypeStructure::TypeAlias(populator) => print_populator::main(populator, &context),

        model::TypeStructure::NamedFields(field_populators) => {
            let contents: proc_macro2::TokenStream = field_populators
                .iter()
                .map(|(field, populator)| {
                    let field = quote::format_ident!("{}", field);
                    let term = print_populator::main(populator, &context);
                    quote::quote! { #field: #term, }
                })
                .collect();

            quote::quote! { #type_ { #contents } }
        }

        model::TypeStructure::TupleFields(populators) => {
            let contents: proc_macro2::TokenStream = populators
                .iter()
                .map(|populator| {
                    let term = print_populator::main(populator, &context);
                    quote::quote! { #term, }
                })
                .collect();

            quote::quote! { #type_(#contents) }
        }
    }
}

fn print_macro(macro_: &str, path: &model::Path) -> proc_macro2::TokenStream {
    let macro_name = quote::format_ident!("{}", macro_);
    let relative_path = &path.relative.0;
    let absolute_path = &path.absolute;

    quote::quote! { #macro_name!(#relative_path, #absolute_path) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod default {
        use super::*;

        #[test]
        fn handles_populator_context() {
            let actual = main(
                &model::View {
                    type_: quote::format_ident!("Asset"),
                    initializer: model::Initializer::Default(model::TypeStructure::TupleFields(
                        vec![
                            model::Populator::RelativePath,
                            model::Populator::ContentsStr,
                        ],
                    )),
                    ..model::stubs::view()
                },
                &model::Path {
                    relative: model::RelativePath::from("b"),
                    absolute: String::from("/a/b"),
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
                    &model::stubs::path(),
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
                            model::Populator::ContentsBytes,
                        )),
                        ..model::stubs::view()
                    },
                    &model::Path {
                        absolute: String::from("/a/b"),
                        ..model::stubs::path()
                    },
                );

                let actual = actual.to_string();
                let expected = quote::quote! { include_bytes!("/a/b") }.to_string();
                assert_eq!(actual, expected);
            }

            #[test]
            fn handles_named_fields() {
                let actual = main(
                    &model::View {
                        type_: quote::format_ident!("MyNamedFields"),
                        initializer: model::Initializer::Default(
                            model::TypeStructure::NamedFields(vec![(
                                String::from("abc"),
                                model::Populator::ContentsStr,
                            )]),
                        ),
                        ..model::stubs::view()
                    },
                    &model::Path {
                        absolute: String::from("/a/b"),
                        ..model::stubs::path()
                    },
                );

                let actual = actual.to_string();
                let expected = quote::quote! {
                    MyNamedFields {
                        abc: include_str!("/a/b"),
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
                            model::TypeStructure::TupleFields(vec![model::Populator::RelativePath]),
                        ),
                        ..model::stubs::view()
                    },
                    &model::Path {
                        relative: model::RelativePath::from("b"),
                        ..model::stubs::path()
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
            &model::Path {
                relative: model::RelativePath::from("b"),
                absolute: String::from("/a/b"),
            },
        );

        let actual = actual.to_string();
        let expected = quote::quote! { abc!("b", "/a/b") }.to_string();
        assert_eq!(actual, expected);
    }
}
