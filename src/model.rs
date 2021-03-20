use std::collections;
use std::path;

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
    pub files: FileForest,
}

pub type FileForest = collections::BTreeMap<String, FileTree>;

pub enum FileTree {
    File { platform_path: path::PathBuf },
    Folder(FileForest),
}
