use crate::model;

pub fn main(file_index: model::FileIndex) -> proc_macro2::TokenStream {
    print_forest(&file_index.resource_type, &file_index.forest)
}

fn print_forest(resource_type: &str, forest: &model::FileForest) -> proc_macro2::TokenStream {
    let trees: proc_macro2::TokenStream = forest
        .iter()
        .map(|(name, tree)| print_tree(&resource_type, name, tree))
        .collect();

    let resource_type = quote::format_ident!("{}", resource_type);
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
    let name = quote::format_ident!("{}", name);
    let resource_type = quote::format_ident!("{}", resource_type);
    let absolute_path = file.absolute_path.to_string_lossy();
    quote::quote! {
        pub const #name: #resource_type = include_str!(#absolute_path);
    }
}

fn print_folder(
    resource_type: &str,
    name: &str,
    forest: &model::FileForest,
) -> proc_macro2::TokenStream {
    let name = quote::format_ident!("{}", name);
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
            resource_type: String::from("Resource"),
            forest,
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            use super::Resource;
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn prints_files() {
        let mut forest = model::FileForest::new();
        forest.insert(
            String::from("MENU_JSON"),
            model::FileTree::File(model::File {
                absolute_path: path::PathBuf::from("/menu.json"),
                ..model::stubs::file()
            }),
        );
        forest.insert(
            String::from("TRANSLATIONS_CSV"),
            model::FileTree::File(model::File {
                absolute_path: path::PathBuf::from("/translations.csv"),
                ..model::stubs::file()
            }),
        );

        let actual = main(model::FileIndex {
            resource_type: String::from("Resource"),
            forest,
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            use super::Resource;

            pub const MENU_JSON: Resource = include_str!("/menu.json");

            pub const TRANSLATIONS_CSV: Resource = include_str!("/translations.csv");
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn prints_folders() {
        let mut levels = model::FileForest::new();
        levels.insert(
            String::from("TUTORIAL_JSON"),
            model::FileTree::File(model::File {
                absolute_path: path::PathBuf::from("/world/levels/tutorial.json"),
                ..model::stubs::file()
            }),
        );
        let mut world = model::FileForest::new();
        world.insert(String::from("levels"), model::FileTree::Folder(levels));
        world.insert(
            String::from("PHYSICAL_CONSTANTS_JSON"),
            model::FileTree::File(model::File {
                absolute_path: path::PathBuf::from("/world/physical_constants.json"),
                ..model::stubs::file()
            }),
        );
        let mut forest = model::FileForest::new();
        forest.insert(
            String::from("CREDITS_MD"),
            model::FileTree::File(model::File {
                absolute_path: path::PathBuf::from("/credits.md"),
                ..model::stubs::file()
            }),
        );
        forest.insert(String::from("world"), model::FileTree::Folder(world));

        let actual = main(model::FileIndex {
            resource_type: String::from("Resource"),
            forest,
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            use super::Resource;

            pub const CREDITS_MD: Resource = include_str!("/credits.md");

            pub mod world {
                use super::Resource;

                pub const PHYSICAL_CONSTANTS_JSON: Resource =
                    include_str!("/world/physical_constants.json");

                pub mod levels {
                    use super::Resource;

                    pub const TUTORIAL_JSON: Resource = include_str!("/world/levels/tutorial.json");
                }
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn prints_both_normal_and_raw_identifiers() {
        let mut raw = model::FileForest::new();
        raw.insert(
            String::from("NORMAL"),
            model::FileTree::File(model::File {
                absolute_path: path::PathBuf::from("/normal"),
                ..model::stubs::file()
            }),
        );
        let mut forest = model::FileForest::new();
        forest.insert(String::from("r#match"), model::FileTree::Folder(raw));

        let actual = main(model::FileIndex {
            resource_type: String::from("Resource"),
            forest,
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            use super::Resource;

            pub mod r#match {
                use super::Resource;

                pub const NORMAL: Resource = include_str!("/normal");
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
