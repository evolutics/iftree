use super::count_files;
use super::print_initializer;
use crate::data;
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
            let name = quote::format_ident!("{}", data::ASSET_ARRAY_NAME);
            let type_ = &view.type_;
            let length = count_files::main(&view.forest);
            quote::quote! { pub static #name: [#type_; #length] = [#contents]; }
        }

        model::Visitor::Identifiers => {
            let name = quote::format_ident!("{}", data::BASE_MODULE_NAME);
            quote::quote! { pub mod #name { #contents } }
        }
    }
}

struct Context<'a> {
    type_: &'a syn::Ident,
    visitor: &'a model::Visitor,
    depth: usize,
}

fn print_forest(context: &Context, forest: &model::FileForest) -> proc_macro2::TokenStream {
    forest
        .values()
        .map(|tree| match tree {
            model::FileTree::File(file) => print_file(context, file),
            model::FileTree::Folder(folder) => print_folder(context, folder),
        })
        .collect()
}

fn print_file(context: &Context, file: &model::File) -> proc_macro2::TokenStream {
    match context.visitor {
        model::Visitor::Array(initializer) => {
            let element = print_initializer::main(context.type_, &initializer, file);
            quote::quote! { #element, }
        }

        model::Visitor::Identifiers => {
            let identifier = &file.identifier;
            let root_path = iter::repeat(quote::quote! { super:: })
                .take(context.depth + 1)
                .collect::<proc_macro2::TokenStream>();
            let type_ = context.type_;
            let array = quote::format_ident!("{}", data::ASSET_ARRAY_NAME);
            let index = file.index;
            quote::quote! { pub static #identifier: &#root_path#type_ = &#root_path#array[#index]; }
        }
    }
}

fn print_folder(context: &Context, folder: &model::Folder) -> proc_macro2::TokenStream {
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
            quote::quote! { pub mod #identifier { #contents } }
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
                forest: vec![
                    (
                        String::from('0'),
                        model::FileTree::File(model::File {
                            relative_path: model::RelativePath::from("a"),
                            ..model::stubs::file()
                        }),
                    ),
                    (
                        String::from('1'),
                        model::FileTree::Folder(model::Folder {
                            forest: vec![(
                                String::from('2'),
                                model::FileTree::File(model::File {
                                    relative_path: model::RelativePath::from("b/c"),
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
                    forest: model::FileForest::new(),
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
                    forest: vec![
                        (
                            String::from('0'),
                            model::FileTree::File(model::File {
                                identifier: quote::format_ident!("A"),
                                index: 1,
                                ..model::stubs::file()
                            }),
                        ),
                        (
                            String::from('1'),
                            model::FileTree::File(model::File {
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
                    pub static A: &super::Asset = &super::ASSETS[1usize];

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
                    forest: vec![
                        (
                            String::from('0'),
                            model::FileTree::File(model::File {
                                identifier: quote::format_ident!("A"),
                                index: 0,
                                ..model::stubs::file()
                            }),
                        ),
                        (
                            String::from('1'),
                            model::FileTree::Folder(model::Folder {
                                identifier: quote::format_ident!("b"),
                                forest: vec![
                                    (
                                        String::from('2'),
                                        model::FileTree::Folder(model::Folder {
                                            identifier: quote::format_ident!("a"),
                                            forest: vec![(
                                                String::from('3'),
                                                model::FileTree::File(model::File {
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
                                        String::from('4'),
                                        model::FileTree::File(model::File {
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
                    pub static A: &super::Asset = &super::ASSETS[0usize];

                    pub mod b {
                        pub mod a {
                            pub static B: &super::super::super::Asset =
                                &super::super::super::ASSETS[2usize];
                        }

                        pub static C: &super::super::Asset = &super::super::ASSETS[1usize];
                    }
                }
            }
            .to_string();
            assert_eq!(actual, expected);
        }
    }
}
