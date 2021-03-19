#[proc_macro_attribute]
pub fn embed_files_as_modules(
    _attribute: proc_macro::TokenStream,
    raw_input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse(raw_input).unwrap();
    generate(&input)
}

fn generate(input: &syn::DeriveInput) -> proc_macro::TokenStream {
    let resource_type = &input.ident;

    let raw_output = quote::quote! {
        #input

        pub mod resources {
            use super::#resource_type;

            pub mod configuration {
                use super::#resource_type;

                pub const MENU_JSON: #resource_type = #resource_type {
                    get: include_str!("resources/configuration/menu.json"),
                };

                pub const TRANSLATIONS_CSV: #resource_type = #resource_type {
                    get: include_str!("resources/configuration/translations.csv"),
                };
            }

            pub const CREDITS_MD: #resource_type = #resource_type {
                get: include_str!("resources/credits.md"),
            };

            pub mod world {
                use super::#resource_type;

                pub mod levels {
                    use super::#resource_type;

                    pub const TUTORIAL_JSON: #resource_type = #resource_type {
                        get: include_str!("resources/world/levels/tutorial.json"),
                    };
                }

                pub const PHYSICAL_CONSTANTS_JSON: #resource_type = #resource_type {
                    get: include_str!("resources/world/physical_constants.json"),
                };
            }
        }
    };

    raw_output.into()
}
