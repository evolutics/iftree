use std::cmp;
use std::collections;
use std::env;
use std::path;
use std::result;
use std::vec;

pub struct Input {
    pub parameters: proc_macro::TokenStream,
    pub item: proc_macro::TokenStream,
}

pub type Output = proc_macro::TokenStream;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    EnvironmentVariable(EnvironmentVariableError),
    Ignore(ignore::Error),
    NameCollisions(vec::Vec<NameCollision>),
    PathStripPrefix(path::StripPrefixError),
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct EnvironmentVariableError {
    pub name: String,
    pub source: env::VarError,
    pub appendix: Option<String>,
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct NameCollision {
    pub colliding_file: File,
    pub existing_filename: Option<String>,
    pub identifier: String,
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct Configuration {
    pub resource_paths: String,
    pub resolve_name_collisions: bool,
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

#[derive(Clone, cmp::Eq, Debug)]
pub struct File {
    pub relative_path: path::PathBuf,
    pub absolute_path: path::PathBuf,
}

#[cfg(test)]
pub mod stubs {
    use super::*;

    pub fn configuration() -> Configuration {
        Configuration {
            resource_paths: String::from("!*"),
            resolve_name_collisions: false,
        }
    }

    pub fn resource_type() -> ResourceType {
        ResourceType {
            identifier: String::from("foo"),
            structure: resource_structure(),
        }
    }

    pub fn resource_structure() -> ResourceStructure {
        ResourceStructure::TypeAlias
    }

    pub fn file() -> File {
        File {
            relative_path: path::PathBuf::from("bar"),
            absolute_path: path::PathBuf::from("/foo/bar"),
        }
    }
}
