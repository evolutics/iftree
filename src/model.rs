use std::cmp;
use std::collections;
use std::env;
use std::path;
use std::result;

pub struct Input {
    pub parameters: proc_macro::TokenStream,
    pub item: proc_macro::TokenStream,
}

pub type Output = proc_macro::TokenStream;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    EnvironmentVariableCargoManifestDir(env::VarError),
    Ignore(ignore::Error),
    PathStripPrefix(path::StripPrefixError),
}

#[derive(Clone, cmp::PartialEq, Debug, serde::Deserialize)]
pub struct Configuration {
    pub resource_folder: path::PathBuf,
    #[serde(default)]
    pub path_filter: String,
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct ResourceType {
    pub identifier: String,
    pub structure: ResourceStructure,
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum ResourceStructure {
    TypeAlias,
    #[allow(dead_code)]
    NamedFields(collections::HashSet<String>),
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
    pub relative_path: path::PathBuf,
    pub absolute_path: path::PathBuf,
}

#[cfg(test)]
pub mod stubs {
    use super::*;

    pub fn configuration() -> Configuration {
        Configuration {
            resource_folder: path::PathBuf::from("foo"),
            path_filter: String::from("bar"),
        }
    }

    pub fn file() -> File {
        File {
            relative_path: path::PathBuf::from("bar"),
            absolute_path: path::PathBuf::from("/foo/bar"),
        }
    }
}
