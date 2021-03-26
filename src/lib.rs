mod error;
mod index_files;
mod model;
mod parse;
mod print;

#[proc_macro_attribute]
pub fn embed_files_as_modules(
    parameters: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    process(model::Input { parameters, item })
}

fn process(input: model::Input) -> model::Output {
    let parameters = input.parameters;
    let configuration = syn::parse_macro_input!(parameters);

    let item = input.item;
    let item_clone = item.clone();
    let resource_type = syn::parse_macro_input!(item);

    match index_files::main(configuration, resource_type)
        .map(|file_index| print::main(item_clone, file_index))
    {
        Err(error) => panic!("{}", error),
        Ok(value) => value,
    }
}
