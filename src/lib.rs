#[proc_macro_attribute]
pub fn embed_files_as_modules(
    _attribute: proc_macro::TokenStream,
    raw_input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse(raw_input).unwrap();
    generate(&input)
}

fn generate(input: &syn::DeriveInput) -> proc_macro::TokenStream {
    let raw_output = quote::quote! {
        #input

        pub mod resources {
            use super::Resource;

            pub mod configuration {
                use super::Resource;

                pub const MENU_JSON: Resource = Resource {
                    get: include_str!("resources/configuration/menu.json"),
                };

                pub const TRANSLATIONS_CSV: Resource = Resource {
                    get: include_str!("resources/configuration/translations.csv"),
                };
            }

            pub const CREDITS_MD: Resource = Resource {
                get: include_str!("resources/credits.md"),
            };

            pub mod world {
                use super::Resource;

                pub mod levels {
                    use super::Resource;

                    pub const TUTORIAL_JSON: Resource = Resource {
                        get: include_str!("resources/world/levels/tutorial.json"),
                    };
                }

                pub const PHYSICAL_CONSTANTS_JSON: Resource = Resource {
                    get: include_str!("resources/world/physical_constants.json"),
                };
            }
        }
    };

    raw_output.into()
}
