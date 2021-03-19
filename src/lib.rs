use syn::parse;

#[proc_macro_attribute]
pub fn embed_files_as_modules(
    _attribute: proc_macro::TokenStream,
    raw_input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let raw_input = proc_macro2::TokenStream::from(raw_input);
    let input = syn::parse2(raw_input.clone()).unwrap();
    generate(raw_input, &input)
}

struct TypeAlias {
    identifier: syn::Ident,
}

impl parse::Parse for TypeAlias {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        input.call(syn::Attribute::parse_outer)?;
        input.parse::<syn::Visibility>()?;
        input.parse::<syn::Token![type]>()?;
        let identifier = input.parse::<syn::Ident>()?;
        input.parse::<syn::Token![=]>()?;
        input.parse::<syn::Type>()?;
        input.parse::<syn::Token![;]>()?;

        Ok(TypeAlias { identifier })
    }
}

fn generate(raw_input: proc_macro2::TokenStream, input: &TypeAlias) -> proc_macro::TokenStream {
    let resource_type = &input.identifier;

    let raw_output = quote::quote! {
        #raw_input

        pub mod resources {
            use super::#resource_type;

            pub mod configuration {
                use super::#resource_type;

                pub const MENU_JSON: #resource_type =
                    include_str!("resources/configuration/menu.json");

                pub const TRANSLATIONS_CSV: #resource_type =
                    include_str!("resources/configuration/translations.csv");
            }

            pub const CREDITS_MD: #resource_type = include_str!("resources/credits.md");

            pub mod world {
                use super::#resource_type;

                pub mod levels {
                    use super::#resource_type;

                    pub const TUTORIAL_JSON: #resource_type =
                        include_str!("resources/world/levels/tutorial.json");
                }

                pub const PHYSICAL_CONSTANTS_JSON: #resource_type =
                    include_str!("resources/world/physical_constants.json");
            }
        }
    };

    raw_output.into()
}
