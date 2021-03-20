pub struct Input {
    pub _attribute: proc_macro::TokenStream,
    pub item: proc_macro::TokenStream,
}

pub type Output = proc_macro::TokenStream;

pub struct TypeAlias {
    pub identifier: syn::Ident,
}

pub struct FileIndex {
    pub resource_type: String,
}
