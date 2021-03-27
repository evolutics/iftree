use super::print_forest_as_modules;
use crate::model;

pub fn main(item: proc_macro::TokenStream, file_index: model::FileIndex) -> model::Output {
    let item = proc_macro2::TokenStream::from(item);
    let file_modules = print_forest_as_modules::main(file_index);

    let output = quote::quote! {
        #item

        pub mod root {
            #file_modules
        }
    };

    output.into()
}
