use super::print_resource_term;
use super::visit_file_forest;
use crate::model;
use std::vec;

pub fn main(file_index: &model::FileIndex) -> proc_macro2::TokenStream {
    let mut stack = vec![proc_macro2::TokenStream::new()];
    visit_file_forest::main(&file_index.resource_type, &file_index.forest, &mut stack);
    stack.pop().unwrap()
}

impl visit_file_forest::Visitor<'_> for model::ResourceType<model::Template> {
    type State = vec::Vec<proc_macro2::TokenStream>;

    fn file(&self, file: &model::File, path: &[&str], stack: &mut Self::State) {
        let name = quote::format_ident!("{}", path.last().unwrap());
        let root_path = path.iter().map(|_| quote::quote! { super:: }).collect();
        let type_identifier = &self.identifier;
        let term = print_resource_term::main(self, file, &root_path);

        let tokens = quote::quote! {
            pub static #name: #root_path#type_identifier = #term;
        };

        stack.last_mut().unwrap().extend(tokens);
    }

    fn before_forest(&self, _path: &[&str], stack: &mut Self::State) {
        stack.push(proc_macro2::TokenStream::new());
    }

    fn after_forest(&self, path: &[&str], stack: &mut Self::State) {
        let name = path.last().unwrap_or(&"base");
        let name = quote::format_ident!("{}", name);
        let trees = stack.pop().unwrap();

        let tokens = quote::quote! {
            pub mod #name {
                #trees
            }
        };

        stack.last_mut().unwrap().extend(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path;

    #[test]
    fn prints_empty_set() {
        let forest = model::FileForest::new();

        let actual = main(&model::FileIndex {
            resource_type: model::ResourceType {
                identifier: quote::format_ident!("Resource"),
                ..model::stubs::resource_type()
            },
            forest,
            ..model::stubs::file_index()
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub mod base {
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn prints_files() {
        let forest = vec![
            (
                String::from("MENU_JSON"),
                model::FileTree::File(model::File {
                    absolute_path: path::PathBuf::from("/menu.json"),
                    ..model::stubs::file()
                }),
            ),
            (
                String::from("TRANSLATIONS_CSV"),
                model::FileTree::File(model::File {
                    absolute_path: path::PathBuf::from("/translations.csv"),
                    ..model::stubs::file()
                }),
            ),
        ]
        .into_iter()
        .collect();

        let actual = main(&model::FileIndex {
            resource_type: model::ResourceType {
                identifier: quote::format_ident!("Resource"),
                structure: model::ResourceStructure::TypeAlias(model::Template::Content),
            },
            forest,
            ..model::stubs::file_index()
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub mod base {
                pub static MENU_JSON: super::Resource = include_str!("/menu.json");

                pub static TRANSLATIONS_CSV: super::Resource = include_str!("/translations.csv");
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn prints_folders() {
        let forest = vec![
            (
                String::from("CREDITS_MD"),
                model::FileTree::File(model::File {
                    absolute_path: path::PathBuf::from("/credits.md"),
                    ..model::stubs::file()
                }),
            ),
            (
                String::from("world"),
                model::FileTree::Folder(
                    vec![
                        (
                            String::from("levels"),
                            model::FileTree::Folder(
                                vec![(
                                    String::from("TUTORIAL_JSON"),
                                    model::FileTree::File(model::File {
                                        absolute_path: path::PathBuf::from(
                                            "/world/levels/tutorial.json",
                                        ),
                                        ..model::stubs::file()
                                    }),
                                )]
                                .into_iter()
                                .collect(),
                            ),
                        ),
                        (
                            String::from("PHYSICAL_CONSTANTS_JSON"),
                            model::FileTree::File(model::File {
                                absolute_path: path::PathBuf::from(
                                    "/world/physical_constants.json",
                                ),
                                ..model::stubs::file()
                            }),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                ),
            ),
        ]
        .into_iter()
        .collect();

        let actual = main(&model::FileIndex {
            resource_type: model::ResourceType {
                identifier: quote::format_ident!("Resource"),
                structure: model::ResourceStructure::TypeAlias(model::Template::Content),
            },
            forest,
            ..model::stubs::file_index()
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub mod base {
                pub static CREDITS_MD: super::Resource = include_str!("/credits.md");

                pub mod world {
                    pub static PHYSICAL_CONSTANTS_JSON: super::super::Resource =
                        include_str!("/world/physical_constants.json");

                    pub mod levels {
                        pub static TUTORIAL_JSON: super::super::super::Resource =
                            include_str!("/world/levels/tutorial.json");
                    }
                }
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn prints_both_normal_and_raw_identifiers() {
        let forest = vec![(
            String::from("r#match"),
            model::FileTree::Folder(
                vec![(
                    String::from("NORMAL"),
                    model::FileTree::File(model::File {
                        absolute_path: path::PathBuf::from("/normal"),
                        ..model::stubs::file()
                    }),
                )]
                .into_iter()
                .collect(),
            ),
        )]
        .into_iter()
        .collect();

        let actual = main(&model::FileIndex {
            resource_type: model::ResourceType {
                identifier: quote::format_ident!("Resource"),
                structure: model::ResourceStructure::TypeAlias(model::Template::Content),
            },
            forest,
            ..model::stubs::file_index()
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub mod base {
                pub mod r#match {
                    pub static NORMAL: super::super::Resource = include_str!("/normal");
                }
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
