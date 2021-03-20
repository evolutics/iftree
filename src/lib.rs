mod index_files;
mod model;
mod parse;
mod print;

#[proc_macro_attribute]
pub fn embed_files_as_modules(
    _attribute: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    process(model::Input { _attribute, item })
}

fn process(input: model::Input) -> model::Output {
    let item = input.item;
    let item_clone = item.clone();
    let resource_type = syn::parse_macro_input!(item);
    let file_index = index_files::main(resource_type);
    print::main(item_clone, file_index)
}
