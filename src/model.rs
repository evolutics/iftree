use std::cmp;
use std::collections;
use std::path;

pub struct Input {
    pub _attribute: proc_macro::TokenStream,
    pub item: proc_macro::TokenStream,
}

pub type Output = proc_macro::TokenStream;

#[derive(cmp::PartialEq, Debug)]
pub struct TypeAlias {
    pub identifier: syn::Ident,
}

#[derive(cmp::PartialEq, Debug)]
pub struct FileIndex {
    pub resource_type: String,
    pub forest: FileForest,
}

pub type FileForest = collections::BTreeMap<String, FileTree>;

#[derive(cmp::PartialEq, Debug)]
pub enum FileTree {
    File(File),
    Folder(FileForest),
}

#[derive(cmp::PartialEq, Debug)]
pub struct File {
    pub full_path: path::PathBuf,
}
