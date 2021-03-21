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
    pub forest: FileForest,
}

pub type FileForest = collections::BTreeMap<String, FileTree>;

pub enum FileTree {
    File(File),
    Folder(FileForest),
}

pub struct File {
    pub path: path::PathBuf,
}
