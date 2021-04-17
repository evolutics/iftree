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
    pub paths: String,
    pub base_folder: path::PathBuf,
    pub root_folder_variable: String,

    pub module_tree: bool,

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

    Custom { macro_: String },
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct Type<T> {
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
    pub type_: Type<Template>,
    pub array: vec::Vec<File>,
    pub forest: Option<FileForest>,
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct File {
    pub relative_path: RelativePath,
    pub absolute_path: path::PathBuf,
}

pub type FileForest = collections::BTreeMap<String, FileTree>;

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum FileTree {
    File { index: usize },
    Folder(FileForest),
}

#[derive(Clone, cmp::Eq, cmp::Ord, cmp::PartialEq, cmp::PartialOrd, Debug)]
pub struct RelativePath(pub String);

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum Error {
    EnvironmentVariable {
        name: String,
        source: env::VarError,
    },

    Ignore(IgnoreError),

    MissingFieldTemplate(FieldIdentifier),

    NameCollision {
        identifier: String,
        competitors: vec::Vec<RelativePath>,
    },

    PathStripPrefix(path::StripPrefixError),
}

#[derive(Clone, Debug)]
pub struct IgnoreError(pub ignore::Error);

#[cfg(test)]
pub mod stubs {
    use super::*;

    pub fn configuration() -> Configuration {
        Configuration {
            paths: String::from("!*"),
            base_folder: path::PathBuf::from("foo"),
            root_folder_variable: String::from("BAR"),

            module_tree: false,

            field_templates: FieldTemplates::new(),
        }
    }

    pub fn type_<T>() -> Type<T> {
        Type {
            identifier: quote::format_ident!("Foo"),
            structure: ResourceStructure::Unit,
        }
    }

    pub fn file_index() -> FileIndex {
        FileIndex {
            type_: type_(),
            array: vec![],
            forest: None,
        }
    }

    pub fn file() -> File {
        File {
            relative_path: RelativePath::from("bar"),
            absolute_path: path::PathBuf::from("/foo/bar"),
        }
    }
}
