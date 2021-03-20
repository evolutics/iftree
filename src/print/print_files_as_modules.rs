use crate::model;
use std::path;

pub fn main(file_index: model::FileIndex) -> proc_macro2::TokenStream {
    print_forest(&file_index.resource_type, &file_index.files)
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
        model::FileTree::File { platform_path } => print_file(resource_type, name, &platform_path),
        model::FileTree::Folder(forest) => print_folder(resource_type, name, forest),
    }
}

fn print_file(
    resource_type: &str,
    name: &str,
    platform_path: &path::PathBuf,
) -> proc_macro2::TokenStream {
    let name = identifier(name);
    let resource_type = identifier(resource_type);
    let platform_path = platform_path.to_string_lossy();
    quote::quote! {
        pub const #name: #resource_type = include_str!(#platform_path);
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
    use crate::model;

    #[test]
    fn prints_empty_set() {
        let files = model::FileForest::new();

        let actual = main(model::FileIndex {
            resource_type: "Resource".to_owned(),
            files,
        });

        let expected = quote::quote! {
            use super::Resource;
        };
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn prints_files() {
        let mut files = model::FileForest::new();
        files.insert(
            "MENU_JSON".to_owned(),
            model::FileTree::File {
                platform_path: path::PathBuf::from("menu.json"),
            },
        );
        files.insert(
            "TRANSLATIONS_CSV".to_owned(),
            model::FileTree::File {
                platform_path: path::PathBuf::from("translations.csv"),
            },
        );

        let actual = main(model::FileIndex {
            resource_type: "Resource".to_owned(),
            files,
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
            model::FileTree::File {
                platform_path: path::PathBuf::from("world/levels/tutorial.json"),
            },
        );
        let mut world = model::FileForest::new();
        world.insert("levels".to_owned(), model::FileTree::Folder(levels));
        world.insert(
            "PHYSICAL_CONSTANTS_JSON".to_owned(),
            model::FileTree::File {
                platform_path: path::PathBuf::from("world/physical_constants.json"),
            },
        );
        let mut files = model::FileForest::new();
        files.insert(
            "CREDITS_MD".to_owned(),
            model::FileTree::File {
                platform_path: path::PathBuf::from("credits.md"),
            },
        );
        files.insert("world".to_owned(), model::FileTree::Folder(world));

        let actual = main(model::FileIndex {
            resource_type: "Resource".to_owned(),
            files,
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
