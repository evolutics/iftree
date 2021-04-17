use crate::data;
use crate::model;
use std::iter;

pub fn main(view: &model::View) -> proc_macro2::TokenStream {
    match &view.forest {
        None => proc_macro2::TokenStream::new(),
        Some(forest) => go(view, forest),
    }
}

fn go(view: &model::View, forest: &model::FileForest) -> proc_macro2::TokenStream {
    let context = Context {
        type_: &view.type_.identifier,
        name: data::BASE_MODULE_IDENTIFIER,
        depth: 0,
    };
    print_folder(context, forest)
}

struct Context<'a> {
    type_: &'a syn::Ident,
    name: &'a str,
    depth: usize,
}

fn print_folder(context: Context, forest: &model::FileForest) -> proc_macro2::TokenStream {
    let name = quote::format_ident!("{}", context.name);

    let content = forest
        .iter()
        .map(|(name, tree)| {
            let context = Context {
                name,
                depth: context.depth + 1,
                ..context
            };
            match tree {
                model::FileTree::File { index } => print_file(context, *index),
                model::FileTree::Folder(forest) => print_folder(context, forest),
            }
        })
        .collect::<proc_macro2::TokenStream>();

    quote::quote! {
        pub mod #name {
            #content
        }
    }
}

fn print_file(context: Context, index: usize) -> proc_macro2::TokenStream {
    let name = quote::format_ident!("{}", context.name);
    let root_path = iter::repeat(quote::quote! { super:: })
        .take(context.depth)
        .collect::<proc_macro2::TokenStream>();
    let type_ = context.type_;
    let array = quote::format_ident!("{}", data::RESOURCE_ARRAY_IDENTIFIER);

    quote::quote! {
        pub static #name: &#root_path#type_ = &#root_path#array[#index];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_none() {
        let actual = main(&model::View {
            forest: None,
            ..model::stubs::view()
        });

        let actual = actual.to_string();
        let expected = quote::quote! {}.to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_empty_set() {
        let actual = main(&model::View {
            forest: Some(model::FileForest::new()),
            ..model::stubs::view()
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub mod base {
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_files() {
        let actual = main(&model::View {
            type_: model::Type {
                identifier: quote::format_ident!("Resource"),
                ..model::stubs::type_()
            },
            forest: Some(
                vec![
                    (String::from('A'), model::FileTree::File { index: 1 }),
                    (String::from("BC"), model::FileTree::File { index: 0 }),
                ]
                .into_iter()
                .collect(),
            ),
            ..model::stubs::view()
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub mod base {
                pub static A: &super::Resource = &super::ARRAY[1usize];

                pub static BC: &super::Resource = &super::ARRAY[0usize];
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_folders() {
        let actual = main(&model::View {
            type_: model::Type {
                identifier: quote::format_ident!("Resource"),
                ..model::stubs::type_()
            },
            forest: Some(
                vec![
                    (String::from('A'), model::FileTree::File { index: 0 }),
                    (
                        String::from('b'),
                        model::FileTree::Folder(
                            vec![
                                (
                                    String::from('a'),
                                    model::FileTree::Folder(
                                        vec![(
                                            String::from('B'),
                                            model::FileTree::File { index: 1 },
                                        )]
                                        .into_iter()
                                        .collect(),
                                    ),
                                ),
                                (String::from('C'), model::FileTree::File { index: 2 }),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ),
            ..model::stubs::view()
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub mod base {
                pub static A: &super::Resource = &super::ARRAY[0usize];

                pub mod b {
                    pub static C: &super::super::Resource = &super::super::ARRAY[2usize];

                    pub mod a {
                        pub static B: &super::super::super::Resource =
                            &super::super::super::ARRAY[1usize];
                    }
                }
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_both_normal_and_raw_identifiers() {
        let actual = main(&model::View {
            type_: model::Type {
                identifier: quote::format_ident!("Resource"),
                ..model::stubs::type_()
            },
            forest: Some(
                vec![(
                    String::from("r#match"),
                    model::FileTree::Folder(
                        vec![(String::from("NORMAL"), model::FileTree::File { index: 0 })]
                            .into_iter()
                            .collect(),
                    ),
                )]
                .into_iter()
                .collect(),
            ),
            ..model::stubs::view()
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub mod base {
                pub mod r#match {
                    pub static NORMAL: &super::super::Resource = &super::super::ARRAY[0usize];
                }
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
