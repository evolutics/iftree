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

struct FileIndex {
    resource_type: syn::Ident,
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
    let resource_type = syn::parse_macro_input!(item);
    let file_index = index_files(resource_type);
    print(item_clone, file_index)
}

fn index_files(resource_type: TypeAlias) -> FileIndex {
    FileIndex {
        resource_type: resource_type.identifier,
    }
}

fn print(item: proc_macro::TokenStream, file_index: FileIndex) -> Output {
    let item = proc_macro2::TokenStream::from(item);
    let resource_type = file_index.resource_type;

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
