use super::print_resource_term;
use super::visit_file_forest;
use crate::model;
use std::vec;

pub fn main(file_index: &model::FileIndex) -> proc_macro2::TokenStream {
    let resource_type = &file_index.resource_type;
    let resource_type = quote::format_ident!("{}", resource_type);
    let visitor = Visitor { resource_type };

    let mut stack = vec![proc_macro2::TokenStream::new()];
    visit_file_forest::main(&visitor, &file_index.forest, &mut stack);
    stack.pop().unwrap()
}

struct Visitor {
    resource_type: syn::Ident,
}

impl visit_file_forest::Visitor<'_> for Visitor {
    type State = vec::Vec<proc_macro2::TokenStream>;

    fn file(&self, file: &model::File, path: &[&str], stack: &mut Self::State) {
        let name = quote::format_ident!("{}", path.last().unwrap());
        let resource_type = &self.resource_type;
        let term = print_resource_term::main(resource_type, &file.resource_term);

        let tokens = quote::quote! {
            pub static #name: #resource_type = #term;
        };

        stack.last_mut().unwrap().extend(tokens);
    }

    fn before_forest(&self, _path: &[&str], stack: &mut Self::State) {
        stack.push(proc_macro2::TokenStream::new());
    }

    fn after_forest(&self, path: &[&str], stack: &mut Self::State) {
        let name = path.last().unwrap_or(&"base");
        let name = quote::format_ident!("{}", name);
        let resource_type = &self.resource_type;
        let trees = stack.pop().unwrap();

        let tokens = quote::quote! {
            pub mod #name {
                use super::#resource_type;

                #trees
            }
        };

        stack.last_mut().unwrap().extend(tokens);
    }
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
            ..model::stubs::file_index()
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub mod base {
                use super::Resource;
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
                    resource_term: model::ResourceTerm::TypeAlias(quote::quote! {
                        include_str!("/menu.json")
                    }),
                    ..model::stubs::file()
                }),
            ),
            (
                String::from("TRANSLATIONS_CSV"),
                model::FileTree::File(model::File {
                    resource_term: model::ResourceTerm::TypeAlias(quote::quote! {
                        include_str!("/translations.csv")
                    }),
                    ..model::stubs::file()
                }),
            ),
        ]
        .into_iter()
        .collect();

        let actual = main(&model::FileIndex {
            resource_type: String::from("Resource"),
            forest,
            ..model::stubs::file_index()
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub mod base {
                use super::Resource;

                pub static MENU_JSON: Resource = include_str!("/menu.json");

                pub static TRANSLATIONS_CSV: Resource = include_str!("/translations.csv");
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
                    resource_term: model::ResourceTerm::TypeAlias(quote::quote! {
                        include_str!("/credits.md")
                    }),
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
                                        resource_term: model::ResourceTerm::TypeAlias(
                                            quote::quote! {
                                                include_str!("/world/levels/tutorial.json")
                                            },
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
                                resource_term: model::ResourceTerm::TypeAlias(quote::quote! {
                                    include_str!("/world/physical_constants.json")
                                }),
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
            resource_type: String::from("Resource"),
            forest,
            ..model::stubs::file_index()
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub mod base {
                use super::Resource;

                pub static CREDITS_MD: Resource = include_str!("/credits.md");

                pub mod world {
                    use super::Resource;

                    pub static PHYSICAL_CONSTANTS_JSON: Resource =
                        include_str!("/world/physical_constants.json");

                    pub mod levels {
                        use super::Resource;

                        pub static TUTORIAL_JSON: Resource =
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
                        resource_term: model::ResourceTerm::TypeAlias(quote::quote! {
                            include_str!("/normal")
                        }),
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
            resource_type: String::from("Resource"),
            forest,
            ..model::stubs::file_index()
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub mod base {
                use super::Resource;

                pub mod r#match {
                    use super::Resource;

                    pub static NORMAL: Resource = include_str!("/normal");
                }
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
