use super::print_array;
use super::print_identifiers;
use crate::model;

pub fn main(item: proc_macro2::TokenStream, view: model::View) -> proc_macro2::TokenStream {
    let array = print_array::main(&view);
    let identifiers = print_identifiers::main(&view);

    quote::quote! {
        #item

        #array

        #identifiers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path;

    #[test]
    fn handles() {
        let actual = main(
            quote::quote! {
                pub type Asset = &'static str;
            },
            model::View {
                type_: model::Type {
                    identifier: quote::format_ident!("Asset"),
                    structure: model::TypeStructure::TypeAlias(model::Template::Content),
                },
                array: vec![model::File {
                    absolute_path: path::PathBuf::from("/a.b"),
                    ..model::stubs::file()
                }],
                forest: vec![(
                    String::from("base"),
                    model::FileTree::Folder(
                        vec![(String::from("A_B"), model::FileTree::File { index: 0 })]
                            .into_iter()
                            .collect(),
                    ),
                )]
                .into_iter()
                .collect(),
            },
        );

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub type Asset = &'static str;

            pub static ASSETS: [Asset; 1usize] = [
                include_str!("/a.b"),
            ];

            pub mod base {
                pub static A_B: &super::Asset = &super::ASSETS[0usize];
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
