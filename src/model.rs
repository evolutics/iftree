use std::cmp;
use std::collections;
use std::path;
use std::result;

pub struct Input {
    pub _attribute: proc_macro::TokenStream,
    pub item: proc_macro::TokenStream,
}

pub type Output = proc_macro::TokenStream;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct Configuration {
    pub resource_folder: path::PathBuf,
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct TypeAlias {
    pub identifier: String,
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct FileIndex {
    pub resource_type: String,
    pub forest: FileForest,
}

pub type FileForest = collections::BTreeMap<String, FileTree>;

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum FileTree {
    File(File),
    Folder(FileForest),
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct File {
    pub canonical_path: path::PathBuf,
    pub full_path: path::PathBuf,
}

#[cfg(test)]
pub mod stubs {
    use super::*;

    pub fn file() -> File {
        File {
            canonical_path: path::PathBuf::from("bar"),
            full_path: path::PathBuf::from("foo/bar"),
        }
    }
}
