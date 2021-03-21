use crate::model;

pub fn main(file_index: model::FileIndex) -> proc_macro2::TokenStream {
    print_forest(&file_index.resource_type, &file_index.forest)
}

fn print_forest(resource_type: &str, forest: &model::FileForest) -> proc_macro2::TokenStream {
    let trees: proc_macro2::TokenStream = forest
        .iter()
        .map(|(name, tree)| print_tree(&resource_type, name, tree))
        .collect();

    let resource_type = identifier(resource_type);
    quote::quote! {
        use super::#resource_type;

        #trees
    }
}

fn print_tree(resource_type: &str, name: &str, tree: &model::FileTree) -> proc_macro2::TokenStream {
    match tree {
        model::FileTree::File(file) => print_file(resource_type, name, file),
        model::FileTree::Folder(forest) => print_folder(resource_type, name, forest),
    }
}

fn print_file(resource_type: &str, name: &str, file: &model::File) -> proc_macro2::TokenStream {
    let name = identifier(name);
    let resource_type = identifier(resource_type);
    let full_path = file.full_path.to_string_lossy();
    quote::quote! {
        pub const #name: #resource_type = include_str!(#full_path);
    }
}

fn identifier(name: &str) -> syn::Ident {
    syn::Ident::new(name, proc_macro2::Span::call_site())
}

fn print_folder(
    resource_type: &str,
    name: &str,
    forest: &model::FileForest,
) -> proc_macro2::TokenStream {
    let name = identifier(name);
    let forest = print_forest(resource_type, forest);
    quote::quote! {
        pub mod #name {
            #forest
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path;

    #[test]
    fn prints_empty_set() {
        let forest = model::FileForest::new();

        let actual = main(model::FileIndex {
            resource_type: "Resource".to_owned(),
            forest,
        });

        let expected = quote::quote! {
            use super::Resource;
        };
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn prints_files() {
        let mut forest = model::FileForest::new();
        forest.insert(
            "MENU_JSON".to_owned(),
            model::FileTree::File(model::File {
                full_path: path::PathBuf::from("menu.json"),
            }),
        );
        forest.insert(
            "TRANSLATIONS_CSV".to_owned(),
            model::FileTree::File(model::File {
                full_path: path::PathBuf::from("translations.csv"),
            }),
        );

        let actual = main(model::FileIndex {
            resource_type: "Resource".to_owned(),
            forest,
        });

        let expected = quote::quote! {
            use super::Resource;

            pub const MENU_JSON: Resource = include_str!("menu.json");

            pub const TRANSLATIONS_CSV: Resource = include_str!("translations.csv");
        };
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn prints_folders() {
        let mut levels = model::FileForest::new();
        levels.insert(
            "TUTORIAL_JSON".to_owned(),
            model::FileTree::File(model::File {
                full_path: path::PathBuf::from("world/levels/tutorial.json"),
            }),
        );
        let mut world = model::FileForest::new();
        world.insert("levels".to_owned(), model::FileTree::Folder(levels));
        world.insert(
            "PHYSICAL_CONSTANTS_JSON".to_owned(),
            model::FileTree::File(model::File {
                full_path: path::PathBuf::from("world/physical_constants.json"),
            }),
        );
        let mut forest = model::FileForest::new();
        forest.insert(
            "CREDITS_MD".to_owned(),
            model::FileTree::File(model::File {
                full_path: path::PathBuf::from("credits.md"),
            }),
        );
        forest.insert("world".to_owned(), model::FileTree::Folder(world));

        let actual = main(model::FileIndex {
            resource_type: "Resource".to_owned(),
            forest,
        });

        let expected = quote::quote! {
            use super::Resource;

            pub const CREDITS_MD: Resource = include_str!("credits.md");

            pub mod world {
                use super::Resource;

                pub const PHYSICAL_CONSTANTS_JSON: Resource =
                    include_str!("world/physical_constants.json");

                pub mod levels {
                    use super::Resource;

                    pub const TUTORIAL_JSON: Resource = include_str!("world/levels/tutorial.json");
                }
            }
        };
        assert_eq!(actual.to_string(), expected.to_string());
    }
}
