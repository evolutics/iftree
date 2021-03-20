use crate::model;

pub fn main(item: proc_macro::TokenStream, file_index: model::FileIndex) -> model::Output {
    let item = proc_macro2::TokenStream::from(item);
    let resource_type = syn::Ident::new(&file_index.resource_type, proc_macro2::Span::call_site());

    let output = quote::quote! {
        #item

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

    output.into()
}
