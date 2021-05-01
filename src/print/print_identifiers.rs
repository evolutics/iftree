use crate::data;
use crate::model;
use std::iter;

pub fn main(view: &model::View) -> proc_macro2::TokenStream {
    let context = Context {
        type_: &view.type_,
        depth: 0,
    };
    print_forest(context, &view.forest)
}

struct Context<'a> {
    type_: &'a syn::Ident,
    depth: usize,
}

fn print_forest(context: Context, forest: &model::FileForest) -> proc_macro2::TokenStream {
    forest
        .values()
        .map(|tree| match tree {
            model::FileTree::File(file) => print_file(&context, file),
            model::FileTree::Folder(folder) => print_folder(&context, folder),
        })
        .collect()
}

fn print_file(context: &Context, file: &model::File) -> proc_macro2::TokenStream {
    let identifier = &file.identifier;
    let root_path = iter::repeat(quote::quote! { super:: })
        .take(context.depth)
        .collect::<proc_macro2::TokenStream>();
    let type_ = context.type_;
    let array = quote::format_ident!("{}", data::ASSET_ARRAY_NAME);
    let index = file.index;

    quote::quote! { pub static #identifier: &#root_path#type_ = &#root_path#array[#index]; }
}

fn print_folder(context: &Context, folder: &model::Folder) -> proc_macro2::TokenStream {
    let identifier = &folder.identifier;
    let context = Context {
        depth: context.depth + 1,
        ..*context
    };
    let forest = &folder.forest;
    let contents = print_forest(context, forest);

    quote::quote! {
        pub mod #identifier {
            #contents
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_empty_set() {
        let actual = main(&model::View {
            forest: model::FileForest::new(),
            ..model::stubs::view()
        });

        let actual = actual.to_string();
        let expected = quote::quote! {}.to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_files() {
        let actual = main(&model::View {
            type_: quote::format_ident!("Asset"),
            forest: vec![
                (
                    String::from('0'),
                    model::FileTree::File(model::File {
                        identifier: quote::format_ident!("A"),
                        index: 1,
                    }),
                ),
                (
                    String::from('1'),
                    model::FileTree::File(model::File {
                        identifier: quote::format_ident!("BC"),
                        index: 0,
                    }),
                ),
            ]
            .into_iter()
            .collect(),
            ..model::stubs::view()
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub static A: &Asset = &ASSETS[1usize];

            pub static BC: &Asset = &ASSETS[0usize];
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_folders() {
        let actual = main(&model::View {
            type_: quote::format_ident!("Asset"),
            forest: vec![
                (
                    String::from('0'),
                    model::FileTree::File(model::File {
                        identifier: quote::format_ident!("A"),
                        index: 0,
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
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub static A: &Asset = &ASSETS[0usize];

            pub mod b {
                pub mod a {
                    pub static B: &super::super::Asset = &super::super::ASSETS[2usize];
                }

                pub static C: &super::Asset = &super::ASSETS[1usize];
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
