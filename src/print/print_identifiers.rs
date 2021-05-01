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
        .iter()
        .map(|(name, tree)| {
            let name = quote::format_ident!("{}", name);

            match tree {
                model::FileTree::File(file) => print_file(&context, name, file),
                model::FileTree::Folder(folder) => print_folder(&context, name, folder),
            }
        })
        .collect()
}

fn print_file(context: &Context, name: syn::Ident, file: &model::File) -> proc_macro2::TokenStream {
    let root_path = iter::repeat(quote::quote! { super:: })
        .take(context.depth)
        .collect::<proc_macro2::TokenStream>();
    let type_ = context.type_;
    let array = quote::format_ident!("{}", data::ASSET_ARRAY_NAME);
    let index = file.index;

    quote::quote! { pub static #name: &#root_path#type_ = &#root_path#array[#index]; }
}

fn print_folder(
    context: &Context,
    name: syn::Ident,
    folder: &model::Folder,
) -> proc_macro2::TokenStream {
    let context = Context {
        depth: context.depth + 1,
        ..*context
    };
    let forest = &folder.forest;
    let contents = print_forest(context, forest);

    quote::quote! {
        pub mod #name {
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
                    String::from('A'),
                    model::FileTree::File(model::File { index: 1 }),
                ),
                (
                    String::from("BC"),
                    model::FileTree::File(model::File { index: 0 }),
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
                    String::from('A'),
                    model::FileTree::File(model::File { index: 0 }),
                ),
                (
                    String::from('b'),
                    model::FileTree::Folder(model::Folder {
                        forest: vec![
                            (
                                String::from('a'),
                                model::FileTree::Folder(model::Folder {
                                    forest: vec![(
                                        String::from('B'),
                                        model::FileTree::File(model::File { index: 1 }),
                                    )]
                                    .into_iter()
                                    .collect(),
                                }),
                            ),
                            (
                                String::from('C'),
                                model::FileTree::File(model::File { index: 2 }),
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
                pub static C: &super::Asset = &super::ASSETS[2usize];

                pub mod a {
                    pub static B: &super::super::Asset = &super::super::ASSETS[1usize];
                }
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
