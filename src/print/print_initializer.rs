use super::print_populator;
use crate::model;

pub fn main(
    type_: &syn::Ident,
    initializer: &model::Initializer,
    file: &model::File,
) -> proc_macro2::TokenStream {
    match initializer {
        model::Initializer::Default(populators) => print_default(type_, populators, file),
        model::Initializer::Macro(name) => print_macro(name, file),
    }
}

fn print_default(
    type_: &syn::Ident,
    populators: &model::TypeStructure<model::Populator>,
    file: &model::File,
) -> proc_macro2::TokenStream {
    let context = print_populator::Context {
        relative_path: &file.relative_path,
        absolute_path: &file.absolute_path,
    };

    match populators {
        model::TypeStructure::Unit => quote::quote! { #type_ },

        model::TypeStructure::TypeAlias(populator) => print_populator::main(populator, &context),

        model::TypeStructure::NamedFields(field_populators) => {
            let contents: proc_macro2::TokenStream = field_populators
                .iter()
                .map(|(field, populator)| {
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

fn print_macro(macro_: &syn::Path, file: &model::File) -> proc_macro2::TokenStream {
    let relative_path = &file.relative_path;
    let absolute_path = &file.absolute_path;

    quote::quote! { #macro_!(#relative_path, #absolute_path) }
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
                &quote::format_ident!("Asset"),
                &model::Initializer::Default(model::TypeStructure::TupleFields(vec![
                    model::Populator::RelativePath,
                    model::Populator::ContentsStr,
                ])),
                &model::File {
                    relative_path: 'b'.into(),
                    absolute_path: "/a/b".into(),
                    ..model::stubs::file()
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
                    &quote::format_ident!("MyUnit"),
                    &model::Initializer::Default(model::TypeStructure::Unit),
                    &model::stubs::file(),
                );

                let actual = actual.to_string();
                let expected = quote::quote! { MyUnit }.to_string();
                assert_eq!(actual, expected);
            }

            #[test]
            fn handles_type_alias() {
                let actual = main(
                    &quote::format_ident!("Foo"),
                    &model::Initializer::Default(model::TypeStructure::TypeAlias(
                        model::Populator::ContentsBytes,
                    )),
                    &model::File {
                        absolute_path: "/a/b".into(),
                        ..model::stubs::file()
                    },
                );

                let actual = actual.to_string();
                let expected = quote::quote! { include_bytes!("/a/b") }.to_string();
                assert_eq!(actual, expected);
            }

            #[test]
            fn handles_named_fields() {
                let actual = main(
                    &quote::format_ident!("MyNamedFields"),
                    &model::Initializer::Default(model::TypeStructure::NamedFields(vec![(
                        quote::format_ident!("abc"),
                        model::Populator::ContentsStr,
                    )])),
                    &model::File {
                        absolute_path: "/a/b".into(),
                        ..model::stubs::file()
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
                    &quote::format_ident!("MyTupleFields"),
                    &model::Initializer::Default(model::TypeStructure::TupleFields(vec![
                        model::Populator::RelativePath,
                    ])),
                    &model::File {
                        relative_path: 'b'.into(),
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
            &quote::format_ident!("Foo"),
            &model::Initializer::Macro(syn::parse_str("abc").unwrap()),
            &model::File {
                relative_path: 'b'.into(),
                absolute_path: "/a/b".into(),
                ..model::stubs::file()
            },
        );

        let actual = actual.to_string();
        let expected = quote::quote! { abc!("b", "/a/b") }.to_string();
        assert_eq!(actual, expected);
    }
}
