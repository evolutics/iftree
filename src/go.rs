use crate::generate_view;
use crate::list_files;
use crate::model;
use crate::print;

pub fn main(
    configuration: model::Configuration,
    item: proc_macro2::TokenStream,
    type_: model::Type<()>,
) -> model::Result<proc_macro2::TokenStream> {
    let files = list_files::main(&configuration)?;
    let view = generate_view::main(&configuration, type_, files)?;
    Ok(print::main(item, view))
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
                identifiers: true,
                field_templates: vec![(model::Field::Anonymous, model::Template::RelativePath)]
                    .into_iter()
                    .collect(),
            },
            quote::quote! {
                pub type Asset = &'static str;
            },
            model::Type {
                identifier: quote::format_ident!("Asset"),
                structure: model::TypeStructure::TypeAlias(()),
            },
        );

        let actual = actual.unwrap().to_string();
        let expected = quote::quote! {
            pub type Asset = &'static str;

            pub static ASSETS: [Asset; 1usize] = [
                "examples/assets/credits.md",
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
