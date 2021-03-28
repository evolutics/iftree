use super::print_forest_as_modules;
use crate::model;

pub fn main(
    item: proc_macro2::TokenStream,
    file_index: model::FileIndex,
) -> proc_macro2::TokenStream {
    let file_modules = print_forest_as_modules::main(file_index);

    quote::quote! {
        #item

        pub mod root {
            #file_modules
        }
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
        let mut forest = model::FileForest::new();
        forest.insert(
            String::from("CREDITS_MD"),
            model::FileTree::File(model::File {
                absolute_path: path::PathBuf::from("/credits.md"),
                ..model::stubs::file()
            }),
        );

        let actual = main(
            item,
            model::FileIndex {
                resource_type: String::from("Resource"),
                forest,
            },
        );

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub type Resource = &'static str;

            pub mod root {
                use super::Resource;

                pub const CREDITS_MD: Resource = include_str!("/credits.md");
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
