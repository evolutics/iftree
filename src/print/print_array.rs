use super::visit_file_forest;
use crate::model;
use std::vec;

pub fn main(file_index: &model::FileIndex) -> proc_macro2::TokenStream {
    if file_index.generate_array {
        generate(file_index)
    } else {
        proc_macro2::TokenStream::new()
    }
}

fn generate(file_index: &model::FileIndex) -> proc_macro2::TokenStream {
    let visitor = Visitor {};
    let mut array = vec![];
    visit_file_forest::main(&visitor, &file_index.forest, &mut array);

    let resource_type = &file_index.resource_type;
    let resource_type = quote::format_ident!("{}", resource_type);
    array.sort_unstable_by_key(|entry| entry.file);
    let length = array.len();
    let content: proc_macro2::TokenStream = array.into_iter().map(|entry| entry.tokens).collect();

    quote::quote! {
        pub static ARRAY: [&#resource_type; #length] = [
            #content
        ];
    }
}

struct Visitor;

struct Entry<'a> {
    tokens: proc_macro2::TokenStream,
    file: &'a model::File,
}

impl<'a> visit_file_forest::Visitor<'a> for Visitor {
    type State = vec::Vec<Entry<'a>>;

    fn file(&self, file: &'a model::File, path: &[&str], array: &mut Self::State) {
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
            &base#path,
        };

        array.push(Entry { tokens, file });
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
            generate_array: true,
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub static ARRAY: [&Resource; 0usize] = [];
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
                    relative_path: model::RelativePath::from("menu.json"),
                    ..model::stubs::file()
                }),
            ),
            (
                String::from("TRANSLATIONS_CSV"),
                model::FileTree::File(model::File {
                    relative_path: model::RelativePath::from("translations.csv"),
                    ..model::stubs::file()
                }),
            ),
        ]
        .into_iter()
        .collect();

        let actual = main(&model::FileIndex {
            resource_type: String::from("Resource"),
            forest,
            generate_array: true,
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub static ARRAY: [&Resource; 2usize] = [
                &base::MENU_JSON,
                &base::TRANSLATIONS_CSV,
            ];
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
                    relative_path: model::RelativePath::from("credits.md"),
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
                                        relative_path: model::RelativePath::from(
                                            "world/levels/tutorial.json",
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
                                relative_path: model::RelativePath::from(
                                    "world/physical_constants.json",
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
            resource_type: String::from("Resource"),
            forest,
            generate_array: true,
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub static ARRAY: [&Resource; 3usize] = [
                &base::CREDITS_MD,
                &base::world::levels::TUTORIAL_JSON,
                &base::world::PHYSICAL_CONSTANTS_JSON,
            ];
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn prints_ordered_by_relative_path() {
        let forest = vec![
            (
                String::from('X'),
                model::FileTree::File(model::File {
                    relative_path: model::RelativePath::from("B"),
                    ..model::stubs::file()
                }),
            ),
            (
                String::from('Y'),
                model::FileTree::File(model::File {
                    relative_path: model::RelativePath::from("A"),
                    ..model::stubs::file()
                }),
            ),
            (
                String::from('Z'),
                model::FileTree::File(model::File {
                    relative_path: model::RelativePath::from("a"),
                    ..model::stubs::file()
                }),
            ),
        ]
        .into_iter()
        .collect();

        let actual = main(&model::FileIndex {
            resource_type: String::from("Resource"),
            forest,
            generate_array: true,
        });

        let actual = actual.to_string();
        let expected = quote::quote! {
            pub static ARRAY: [&Resource; 3usize] = [
                &base::Y,
                &base::X,
                &base::Z,
            ];
        }
        .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_not_to_generate_it_prints_empty() {
        let actual = main(&model::FileIndex {
            generate_array: false,
            ..model::stubs::file_index()
        });

        let actual = actual.is_empty();
        assert!(actual);
    }
}
