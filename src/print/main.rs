use super::print_forest_as_modules;
use crate::model;

pub fn main(
    item: proc_macro2::TokenStream,
    file_index: model::FileIndex,
) -> proc_macro2::TokenStream {
    let file_modules = print_forest_as_modules::main(file_index);

    quote::quote! {
        #item

        pub mod root {
            #file_modules
        }
    }
}
