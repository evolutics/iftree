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

            pub const CREDITS: Resource = Resource {
                get: include_str!("resources/credits.md"),
            };
        }
    };

    raw_output.into()
}
