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

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum Error {
    EnvironmentVariable(EnvironmentVariableError),
    Ignore(IgnoreError),
    MissingImplementation(FieldIdentifier),
    NameCollisions(vec::Vec<NameCollision>),
    NonStandardTemplate(Template),
    PathStripPrefix(path::StripPrefixError),
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct EnvironmentVariableError {
    pub name: String,
    pub source: env::VarError,
}

#[derive(Clone, Debug)]
pub struct IgnoreError(pub ignore::Error);

#[derive(Clone, cmp::Eq, cmp::Ord, cmp::PartialEq, cmp::PartialOrd, Debug)]
pub enum FieldIdentifier {
    Anonymous,
    Named(String),
    Indexed(usize),
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct NameCollision {
    pub colliding_file: File,
    pub existing_filename: Option<String>,
    pub identifier: String,
}

pub type Template = String;

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct Configuration {
    pub resource_paths: String,
    pub resolve_name_collisions: bool,
    pub base_folder_environment_variable: String,
    pub field_templates: collections::BTreeMap<FieldIdentifier, Template>,
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct ResourceType {
    pub identifier: String,
    pub structure: ResourceTypeStructure,
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum ResourceTypeStructure {
    #[allow(dead_code)]
    Unit,
    TypeAlias,
    #[allow(dead_code)]
    NamedFields(vec::Vec<String>),
    #[allow(dead_code)]
    TupleFields(usize),
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

#[derive(Clone, Debug)]
pub struct File {
    pub relative_path: path::PathBuf,
    pub resource_term: ResourceTerm<proc_macro2::TokenStream>,
}

#[derive(Clone, cmp::Eq, cmp::Ord, cmp::PartialEq, cmp::PartialOrd, Debug)]
pub enum ResourceTerm<T> {
    Unit,
    TypeAlias(T),
    NamedFields(vec::Vec<(String, T)>),
    TupleFields(vec::Vec<T>),
}

#[cfg(test)]
pub mod stubs {
    use super::*;

    pub fn configuration() -> Configuration {
        Configuration {
            resource_paths: String::from("!*"),
            resolve_name_collisions: false,
            base_folder_environment_variable: String::from("FOO"),
            field_templates: Default::default(),
        }
    }

    pub fn file() -> File {
        File {
            relative_path: path::PathBuf::from("bar"),
            resource_term: ResourceTerm::Unit,
        }
    }
}
