use super::print_array;
use super::print_resource_module;
use crate::model;

pub fn main(
    item: proc_macro2::TokenStream,
    file_index: model::FileIndex,
) -> proc_macro2::TokenStream {
    let resource_module = print_resource_module::main(&file_index);
    let array = print_array::main(&file_index);

    quote::quote! {
        #item

        #resource_module

        #array
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path;

    #[test]
    fn prints() {
        let item = quote::quote! {
            pub type Resource = &'static str;
        };
        let forest = vec![(
            String::from("CREDITS_MD"),
            model::FileTree::File(model::File {
                absolute_path: path::PathBuf::from("/credits.md"),
                ..model::stubs::file()
            }),
        )]
        .into_iter()
        .collect();

        let actual = main(
            item,
            model::FileIndex {
                resource_type: model::ResourceType {
                    identifier: quote::format_ident!("Resource"),
                    structure: model::ResourceStructure::TypeAlias(model::Template::Content),
                },
                forest,
                generate_array: true,
            },
        );

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub type Resource = &'static str;

            pub mod base {
                pub static CREDITS_MD: super::Resource = include_str!("/credits.md");
            }

            pub static ARRAY: [&Resource; 1usize] = [
                &base::CREDITS_MD,
            ];
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
