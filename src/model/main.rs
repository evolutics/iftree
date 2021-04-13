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

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct Configuration {
    pub resource_paths: String,
    pub base_folder: path::PathBuf,
    pub root_folder_variable: String,

    pub resolve_name_collisions: bool,
    pub generate_array: bool,

    pub field_templates: FieldTemplates,
}

pub type FieldTemplates = collections::BTreeMap<FieldIdentifier, Template>;

#[derive(Clone, cmp::Eq, cmp::Ord, cmp::PartialEq, cmp::PartialOrd, Debug, serde::Deserialize)]
#[serde(from = "String")]
pub enum FieldIdentifier {
    Anonymous,
    Named(String),
    Indexed(usize),
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum Template {
    Content,
    GetContent,
    GetRawContent,
    RawContent,
    RelativePath,

    Custom(String),
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct ResourceType<T> {
    pub identifier: syn::Ident,
    pub structure: ResourceStructure<T>,
}

#[derive(Clone, cmp::Eq, cmp::Ord, cmp::PartialEq, cmp::PartialOrd, Debug)]
pub enum ResourceStructure<T> {
    Unit,
    TypeAlias(T),
    NamedFields(vec::Vec<(String, T)>),
    TupleFields(vec::Vec<T>),
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct FileIndex {
    pub resource_type: syn::Ident,
    pub forest: FileForest,
    pub generate_array: bool,
}

pub type FileForest = collections::BTreeMap<String, FileTree>;

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum FileTree {
    File(File),
    Folder(FileForest),
}

#[derive(Clone, Debug)]
pub struct File {
    pub relative_path: RelativePath,
    pub resource_term: ResourceTerm,
}

#[derive(Clone, cmp::Eq, cmp::Ord, cmp::PartialEq, cmp::PartialOrd, Debug)]
pub struct RelativePath(pub String);

pub type ResourceTerm = ResourceStructure<proc_macro2::TokenStream>;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum Error {
    EnvironmentVariable(EnvironmentVariableError),
    Ignore(IgnoreError),
    MissingFieldTemplate(FieldIdentifier),
    NameCollisions(vec::Vec<NameCollision>),
    PathStripPrefix(path::StripPrefixError),
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct EnvironmentVariableError {
    pub name: String,
    pub source: env::VarError,
}

#[derive(Clone, Debug)]
pub struct IgnoreError(pub ignore::Error);

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct NameCollision {
    pub colliding_file: File,
    pub existing_filename: Option<String>,
    pub identifier: String,
}

#[cfg(test)]
pub mod stubs {
    use super::*;

    pub fn configuration() -> Configuration {
        Configuration {
            resource_paths: String::from("!*"),
            base_folder: path::PathBuf::from("foo"),
            root_folder_variable: String::from("BAR"),

            resolve_name_collisions: false,
            generate_array: false,

            field_templates: FieldTemplates::new(),
        }
    }

    pub fn file_index() -> FileIndex {
        FileIndex {
            resource_type: quote::format_ident!("Foo"),
            forest: FileForest::new(),
            generate_array: false,
        }
    }

    pub fn file() -> File {
        File {
            relative_path: RelativePath::from("bar"),
            resource_term: ResourceTerm::Unit,
        }
    }
}
