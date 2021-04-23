use crate::generate_view;
use crate::list_files;
use crate::model;
use crate::print;

pub fn main(
    configuration: model::Configuration,
    item: proc_macro2::TokenStream,
    type_: model::Type<()>,
) -> model::Result<proc_macro2::TokenStream> {
    // Pipeline overview:
    // 1. I/O only happens here.
    let files = list_files::main(&configuration)?;
    // 2. Construct a view model.
    let view = generate_view::main(&configuration, type_, files)?;
    // 3. Generate code ("view").
    Ok(print::main(&configuration, item, view))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path;

    #[test]
    fn handles() {
        let actual = main(
            model::Configuration {
                paths: String::from("/examples/assets/credits.md"),
                base_folder: path::PathBuf::new(),
                root_folder_variable: String::from("CARGO_MANIFEST_DIR"),
                initializer: None,
                identifiers: true,
                debug: false,
            },
            quote::quote! {
                pub struct Asset {
                    relative_path: &'static str,
                }
            },
            model::Type {
                name: quote::format_ident!("Asset"),
                structure: model::TypeStructure::NamedFields(vec![(
                    String::from("relative_path"),
                    (),
                )]),
            },
        );

        let actual = actual.unwrap().to_string();
        let expected = quote::quote! {
            pub struct Asset {
                relative_path: &'static str,
            }

            pub static ASSETS: [Asset; 1usize] = [
                Asset {
                    relative_path: "examples/assets/credits.md",
                },
            ];

            pub mod base {
                pub mod r#examples {
                    pub mod r#assets {
                        pub static r#CREDITS_MD: &super::super::super::Asset =
                            &super::super::super::ASSETS[0usize];
                    }
                }
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
