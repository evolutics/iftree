use super::count_files;
use super::print_initializer;
use crate::model;
use std::iter;

pub fn main(view: &model::View, visitor: &model::Visitor) -> proc_macro2::TokenStream {
    let contents = print_forest(
        &Context {
            type_: &view.type_,
            visitor,
            depth: 0,
        },
        &view.forest,
    );

    match visitor {
        model::Visitor::Array(_) => {
            let type_ = &view.type_;
            let length = count_files::main(&view.forest);
            quote::quote! { pub static ASSETS: [#type_; #length] = [#contents]; }
        }

        model::Visitor::Identifiers => quote::quote! { pub mod base { #contents } },

        model::Visitor::Custom(model::CustomVisitor {
            visit_base: None, ..
        }) => contents,

        model::Visitor::Custom(model::CustomVisitor {
            visit_base: Some(macro_),
            ..
        }) => {
            let length = count_files::main(&view.forest);
            quote::quote! { #macro_! { #length, #contents } }
        }
    }
}

struct Context<'a> {
    type_: &'a syn::Ident,
    visitor: &'a model::Visitor,
    depth: usize,
}

fn print_forest(context: &Context, forest: &model::Forest) -> proc_macro2::TokenStream {
    forest
        .iter()
        .map(|(name, tree)| match tree {
            model::Tree::File(file) => print_file(context, name, file),
            model::Tree::Folder(folder) => print_folder(context, name, folder),
        })
        .collect()
}

fn print_file(context: &Context, name: &str, file: &model::File) -> proc_macro2::TokenStream {
    match context.visitor {
        model::Visitor::Array(initializer) => {
            let element = print_initializer::main(context.type_, initializer, file);
            quote::quote! { #element, }
        }

        model::Visitor::Identifiers => {
            let identifier = &file.identifier;
            let root_path = iter::repeat_n(quote::quote! { super:: }, context.depth + 1)
                .collect::<proc_macro2::TokenStream>();
            let type_ = context.type_;
            let index = file.index;
            quote::quote! {
                #[doc = #name]
                pub static #identifier: &#root_path #type_ = &#root_path ASSETS[#index];
            }
        }

        model::Visitor::Custom(model::CustomVisitor { visit_file, .. }) => {
            let id = &file.identifier;
            let index = file.index;
            let relative_path = &file.relative_path;
            let absolute_path = &file.absolute_path;
            quote::quote! { #visit_file! { #name, #id, #index, #relative_path, #absolute_path } }
        }
    }
}

fn print_folder(context: &Context, name: &str, folder: &model::Folder) -> proc_macro2::TokenStream {
    let contents = print_forest(
        &Context {
            depth: context.depth + 1,
            ..*context
        },
        &folder.forest,
    );

    match context.visitor {
        model::Visitor::Array(_) => contents,

        model::Visitor::Identifiers => {
            let identifier = &folder.identifier;
            quote::quote! {
                #[doc = #name]
                pub mod #identifier { #contents }
            }
        }

        model::Visitor::Custom(model::CustomVisitor {
            visit_folder: None, ..
        }) => contents,

        model::Visitor::Custom(model::CustomVisitor {
            visit_folder: Some(macro_),
            ..
        }) => {
            let id = &folder.identifier;
            quote::quote! { #macro_! { #name, #id, #contents } }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_array() {
        let actual = main(
            &model::View {
                type_: quote::format_ident!("Asset"),
                forest: [
                    (
                        "0".into(),
                        model::Tree::File(model::File {
                            relative_path: "a".into(),
                            ..model::stubs::file()
                        }),
                    ),
                    (
                        "1".into(),
                        model::Tree::Folder(model::Folder {
                            forest: [(
                                "2".into(),
                                model::Tree::File(model::File {
                                    relative_path: "b/c".into(),
                                    ..model::stubs::file()
                                }),
                            )]
                            .into_iter()
                            .collect(),
                            ..model::stubs::folder()
                        }),
                    ),
                ]
                .into_iter()
                .collect(),
                ..model::stubs::view()
            },
            &model::Visitor::Array(model::Initializer::Default(
                model::TypeStructure::TypeAlias(model::Populator::RelativePath),
            )),
        );

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub static ASSETS: [Asset; 2usize] = [
                "a",
                "b/c",
            ];
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[cfg(test)]
    mod handles_identifiers {
        use super::*;

        #[test]
        fn handles_empty_set() {
            let actual = main(
                &model::View {
                    forest: model::Forest::new(),
                    ..model::stubs::view()
                },
                &model::Visitor::Identifiers,
            );

            let actual = actual.to_string();
            let expected = quote::quote! { pub mod base {} }.to_string();
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_files() {
            let actual = main(
                &model::View {
                    type_: quote::format_ident!("Asset"),
                    forest: [
                        (
                            "0".into(),
                            model::Tree::File(model::File {
                                identifier: quote::format_ident!("A"),
                                index: 1,
                                ..model::stubs::file()
                            }),
                        ),
                        (
                            "1".into(),
                            model::Tree::File(model::File {
                                identifier: quote::format_ident!("BC"),
                                index: 0,
                                ..model::stubs::file()
                            }),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                    ..model::stubs::view()
                },
                &model::Visitor::Identifiers,
            );

            let actual = actual.to_string();
            let expected = quote::quote! {
                pub mod base {
                    #[doc = "0"]
                    pub static A: &super::Asset = &super::ASSETS[1usize];

                    #[doc = "1"]
                    pub static BC: &super::Asset = &super::ASSETS[0usize];
                }
            }
            .to_string();
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_folders() {
            let actual = main(
                &model::View {
                    type_: quote::format_ident!("Asset"),
                    forest: [
                        (
                            "0".into(),
                            model::Tree::File(model::File {
                                identifier: quote::format_ident!("A"),
                                index: 0,
                                ..model::stubs::file()
                            }),
                        ),
                        (
                            "1".into(),
                            model::Tree::Folder(model::Folder {
                                identifier: quote::format_ident!("b"),
                                forest: [
                                    (
                                        "2".into(),
                                        model::Tree::Folder(model::Folder {
                                            identifier: quote::format_ident!("a"),
                                            forest: [(
                                                "3".into(),
                                                model::Tree::File(model::File {
                                                    identifier: quote::format_ident!("B"),
                                                    index: 2,
                                                    ..model::stubs::file()
                                                }),
                                            )]
                                            .into_iter()
                                            .collect(),
                                        }),
                                    ),
                                    (
                                        "4".into(),
                                        model::Tree::File(model::File {
                                            identifier: quote::format_ident!("C"),
                                            index: 1,
                                            ..model::stubs::file()
                                        }),
                                    ),
                                ]
                                .into_iter()
                                .collect(),
                            }),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                    ..model::stubs::view()
                },
                &model::Visitor::Identifiers,
            );

            let actual = actual.to_string();
            let expected = quote::quote! {
                pub mod base {
                    #[doc = "0"]
                    pub static A: &super::Asset = &super::ASSETS[0usize];

                    #[doc = "1"]
                    pub mod b {
                        #[doc = "2"]
                        pub mod a {
                            #[doc = "3"]
                            pub static B: &super::super::super::Asset =
                                &super::super::super::ASSETS[2usize];
                        }

                        #[doc = "4"]
                        pub static C: &super::super::Asset = &super::super::ASSETS[1usize];
                    }
                }
            }
            .to_string();
            assert_eq!(actual, expected);
        }
    }

    #[cfg(test)]
    mod handles_custom {
        use super::*;

        #[test]
        fn handles_with_options() {
            let actual = main(
                &model::View {
                    forest: [
                        (
                            "0".into(),
                            model::Tree::File(model::File {
                                identifier: quote::format_ident!("A"),
                                index: 0,
                                relative_path: "a".into(),
                                absolute_path: "/a".into(),
                            }),
                        ),
                        (
                            "1".into(),
                            model::Tree::Folder(model::Folder {
                                identifier: quote::format_ident!("b"),
                                forest: [
                                    (
                                        "2".into(),
                                        model::Tree::Folder(model::Folder {
                                            identifier: quote::format_ident!("a"),
                                            forest: [(
                                                "3".into(),
                                                model::Tree::File(model::File {
                                                    identifier: quote::format_ident!("B"),
                                                    index: 2,
                                                    relative_path: "b/a/b".into(),
                                                    absolute_path: "/b/a/b".into(),
                                                }),
                                            )]
                                            .into_iter()
                                            .collect(),
                                        }),
                                    ),
                                    (
                                        "4".into(),
                                        model::Tree::File(model::File {
                                            identifier: quote::format_ident!("C"),
                                            index: 1,
                                            relative_path: "b/c".into(),
                                            absolute_path: "/b/c".into(),
                                        }),
                                    ),
                                ]
                                .into_iter()
                                .collect(),
                            }),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                    ..model::stubs::view()
                },
                &model::Visitor::Custom(model::CustomVisitor {
                    visit_base: Some(syn::parse_str("visit_base").unwrap()),
                    visit_folder: Some(syn::parse_str("visit_folder").unwrap()),
                    visit_file: syn::parse_str("visit_file").unwrap(),
                }),
            );

            let actual = actual.to_string();
            let expected = quote::quote! {
                visit_base! {
                    3usize,
                    visit_file! { "0", A, 0usize, "a", "/a" }
                    visit_folder! {
                        "1",
                        b,
                        visit_folder! {
                            "2",
                            a,
                            visit_file! { "3", B, 2usize, "b/a/b", "/b/a/b" }
                        }
                        visit_file! { "4", C, 1usize, "b/c", "/b/c" }
                    }
                }
            }
            .to_string();
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_without_visit_base() {
            let actual = main(
                &model::View {
                    forest: [(
                        "0".into(),
                        model::Tree::Folder(model::Folder {
                            identifier: quote::format_ident!("a"),
                            forest: [(
                                "1".into(),
                                model::Tree::File(model::File {
                                    identifier: quote::format_ident!("B"),
                                    index: 0,
                                    relative_path: "a/b".into(),
                                    absolute_path: "/a/b".into(),
                                }),
                            )]
                            .into_iter()
                            .collect(),
                        }),
                    )]
                    .into_iter()
                    .collect(),
                    ..model::stubs::view()
                },
                &model::Visitor::Custom(model::CustomVisitor {
                    visit_base: None,
                    visit_folder: Some(syn::parse_str("visit_folder").unwrap()),
                    visit_file: syn::parse_str("visit_file").unwrap(),
                }),
            );

            let actual = actual.to_string();
            let expected = quote::quote! {
                visit_folder! {
                    "0",
                    a,
                    visit_file! { "1", B, 0usize, "a/b", "/a/b" }
                }
            }
            .to_string();
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_without_visit_folder() {
            let actual = main(
                &model::View {
                    forest: [(
                        "0".into(),
                        model::Tree::Folder(model::Folder {
                            identifier: quote::format_ident!("a"),
                            forest: [(
                                "1".into(),
                                model::Tree::File(model::File {
                                    identifier: quote::format_ident!("B"),
                                    index: 0,
                                    relative_path: "a/b".into(),
                                    absolute_path: "/a/b".into(),
                                }),
                            )]
                            .into_iter()
                            .collect(),
                        }),
                    )]
                    .into_iter()
                    .collect(),
                    ..model::stubs::view()
                },
                &model::Visitor::Custom(model::CustomVisitor {
                    visit_base: Some(syn::parse_str("visit_base").unwrap()),
                    visit_folder: None,
                    visit_file: syn::parse_str("visit_file").unwrap(),
                }),
            );

            let actual = actual.to_string();
            let expected = quote::quote! {
                visit_base! {
                    1usize,
                    visit_file! { "1", B, 0usize, "a/b", "/a/b" }
                }
            }
            .to_string();
            assert_eq!(actual, expected);
        }
    }
}
