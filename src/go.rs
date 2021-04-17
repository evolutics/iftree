use crate::index_files;
use crate::inspect_system;
use crate::model;
use crate::print;

pub fn main(
    configuration: model::Configuration,
    item: proc_macro2::TokenStream,
    type_: model::Type<()>,
) -> model::Result<proc_macro2::TokenStream> {
    let system_data = inspect_system::main(&configuration)?;
    let file_index = index_files::main(&configuration, type_, system_data)?;
    Ok(print::main(item, file_index))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path;

    #[test]
    fn handles() {
        let actual = main(
            model::Configuration {
                paths: String::from("/examples/resources/credits.md"),
                base_folder: path::PathBuf::new(),
                root_folder_variable: String::from("CARGO_MANIFEST_DIR"),
                module_tree: true,
                field_templates: vec![(
                    model::FieldIdentifier::Anonymous,
                    model::Template::RelativePath,
                )]
                .into_iter()
                .collect(),
            },
            quote::quote! {
                pub type Resource = &'static str;
            },
            model::Type {
                identifier: quote::format_ident!("Resource"),
                structure: model::TypeStructure::TypeAlias(()),
            },
        );

        let actual = actual.unwrap().to_string();
        let expected = quote::quote! {
            pub type Resource = &'static str;

            pub static ARRAY: [Resource; 1usize] = [
                "examples/resources/credits.md",
            ];

            pub mod base {
                pub mod r#examples {
                    pub mod r#resources {
                        pub static r#CREDITS_MD: &super::super::super::Resource =
                            &super::super::super::ARRAY[0usize];
                    }
                }
            }
        }
        .to_string();
        assert_eq!(actual, expected);
    }
}
