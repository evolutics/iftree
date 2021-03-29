use super::file_forest_visit;
use crate::model;
use std::vec;

pub fn main(file_index: &model::FileIndex) -> proc_macro2::TokenStream {
    let visitor = Visitor {};
    let mut array = vec![];
    file_forest_visit::visit(&visitor, &file_index.forest, &mut array);

    let resource_type = &file_index.resource_type;
    let resource_type = quote::format_ident!("{}", resource_type);
    let length = array.len();
    let content: proc_macro2::TokenStream = array.into_iter().collect();

    quote::quote! {
        pub const ARRAY: [&#resource_type; #length] = [
            #content
        ];
    }
}

struct Visitor;

impl file_forest_visit::Visitor for Visitor {
    type State = vec::Vec<proc_macro2::TokenStream>;

    fn file(&self, _file: &model::File, path: &[&str], array: &mut Self::State) {
        let path: proc_macro2::TokenStream = path
            .iter()
            .map(|name| {
                let name = quote::format_ident!("{}", name);
                quote::quote! {
                    ::#name
                }
            })
            .collect();

        let tokens = quote::quote! {
            &root#path,
        };

        array.push(tokens);
    }

    fn before_forest(&self, _path: &[&str], _array: &mut Self::State) {}

    fn after_forest(&self, _path: &[&str], _array: &mut Self::State) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_empty_set() {
        let forest = model::FileForest::new();

        let actual = main(&model::FileIndex {
            resource_type: String::from("Resource"),
            forest,
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub const ARRAY: [&Resource; 0usize] = [];
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn prints_files() {
        let mut forest = model::FileForest::new();
        forest.insert(
            String::from("MENU_JSON"),
            model::FileTree::File(model::stubs::file()),
        );
        forest.insert(
            String::from("TRANSLATIONS_CSV"),
            model::FileTree::File(model::stubs::file()),
        );

        let actual = main(&model::FileIndex {
            resource_type: String::from("Resource"),
            forest,
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub const ARRAY: [&Resource; 2usize] = [
                &root::MENU_JSON,
                &root::TRANSLATIONS_CSV,
            ];
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn prints_folders() {
        let mut levels = model::FileForest::new();
        levels.insert(
            String::from("TUTORIAL_JSON"),
            model::FileTree::File(model::stubs::file()),
        );
        let mut world = model::FileForest::new();
        world.insert(String::from("levels"), model::FileTree::Folder(levels));
        world.insert(
            String::from("PHYSICAL_CONSTANTS_JSON"),
            model::FileTree::File(model::stubs::file()),
        );
        let mut forest = model::FileForest::new();
        forest.insert(
            String::from("CREDITS_MD"),
            model::FileTree::File(model::stubs::file()),
        );
        forest.insert(String::from("world"), model::FileTree::Folder(world));

        let actual = main(&model::FileIndex {
            resource_type: String::from("Resource"),
            forest,
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub const ARRAY: [&Resource; 3usize] = [
                &root::CREDITS_MD,
                &root::world::PHYSICAL_CONSTANTS_JSON,
                &root::world::levels::TUTORIAL_JSON,
            ];
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
