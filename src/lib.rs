use syn::parse;

#[proc_macro_attribute]
pub fn embed_files_as_modules(
    _attribute: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    process(Input { _attribute, item })
}

struct Input {
    _attribute: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
}

type Output = proc_macro::TokenStream;

struct TypeAlias {
    identifier: syn::Ident,
}

impl parse::Parse for TypeAlias {
    fn parse(item: parse::ParseStream) -> syn::Result<Self> {
        item.call(syn::Attribute::parse_outer)?;
        item.parse::<syn::Visibility>()?;
        item.parse::<syn::Token![type]>()?;
        let identifier = item.parse::<syn::Ident>()?;
        item.parse::<syn::Token![=]>()?;
        item.parse::<syn::Type>()?;
        item.parse::<syn::Token![;]>()?;

        Ok(TypeAlias { identifier })
    }
}

fn process(input: Input) -> Output {
    let item = input.item;
    let item_clone = item.clone();
    let resource = syn::parse_macro_input!(item);
    generate(item_clone, &resource)
}

fn generate(item: proc_macro::TokenStream, resource: &TypeAlias) -> Output {
    let item = proc_macro2::TokenStream::from(item);
    let resource_type = &resource.identifier;

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
