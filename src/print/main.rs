use super::print_array;
use super::print_resource_module;
use crate::model;

pub fn main(
    item: proc_macro2::TokenStream,
    file_index: model::FileIndex,
) -> proc_macro2::TokenStream {
    let array = print_array::main(&file_index);
    let resource_module = print_resource_module::main(&file_index);

    quote::quote! {
        #item

        #array

        #resource_module
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
                pub type Resource = &'static str;
            },
            model::FileIndex {
                type_: model::Type {
                    identifier: quote::format_ident!("Resource"),
                    structure: model::TypeStructure::TypeAlias(model::Template::Content),
                },
                array: vec![model::File {
                    absolute_path: path::PathBuf::from("/a.b"),
                    ..model::stubs::file()
                }],
                forest: Some(
                    vec![(String::from("A_B"), model::FileTree::File { index: 0 })]
                        .into_iter()
                        .collect(),
                ),
            },
        );

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub type Resource = &'static str;

            pub static ARRAY: [Resource; 1usize] = [
                include_str!("/a.b"),
            ];

            pub mod base {
                pub static A_B: &super::Resource = &super::ARRAY[0usize];
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
