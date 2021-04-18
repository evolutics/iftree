use std::cmp;
use std::collections;
use std::env;
use std::path;
use std::result;
use std::vec;

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct Configuration {
    pub paths: String,
    pub base_folder: path::PathBuf,
    pub root_folder_variable: String,

    pub identifiers: bool,
    pub debug: bool,

    pub field_templates: FieldTemplates,
}

pub type FieldTemplates = collections::BTreeMap<Field, Template>;

#[derive(Clone, cmp::Eq, cmp::Ord, cmp::PartialEq, cmp::PartialOrd, Debug, serde::Deserialize)]
#[serde(from = "String")]
pub enum Field {
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
    pub name: syn::Ident,
    pub structure: TypeStructure<T>,
}

#[derive(Clone, cmp::Eq, cmp::Ord, cmp::PartialEq, cmp::PartialOrd, Debug)]
pub enum TypeStructure<T> {
    Unit,
    TypeAlias(T),
    NamedFields(vec::Vec<(String, T)>),
    TupleFields(vec::Vec<T>),
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct File {
    pub relative_path: RelativePath,
    pub absolute_path: String,
}

#[derive(Clone, cmp::Eq, cmp::Ord, cmp::PartialEq, cmp::PartialOrd, Debug)]
pub struct RelativePath(pub String);

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct View {
    pub type_: Type<Template>,
    pub array: vec::Vec<File>,
    pub forest: FileForest,
}

pub type FileForest = collections::BTreeMap<String, FileTree>;

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum FileTree {
    File { index: usize },
    Folder(FileForest),
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum Error {
    EnvironmentVariable {
        name: String,
        source: env::VarError,
    },

    Ignore(IgnoreError),

    MissingFieldTemplate(Field),

    NameCollision {
        name: String,
        competitors: vec::Vec<RelativePath>,
    },

    PathInvalidUnicode(path::PathBuf),

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

            identifiers: false,
            debug: false,

            field_templates: FieldTemplates::new(),
        }
    }

    pub fn type_<T>() -> Type<T> {
        Type {
            name: quote::format_ident!("Foo"),
            structure: TypeStructure::Unit,
        }
    }

    pub fn file() -> File {
        File {
            relative_path: RelativePath::from("bar"),
            absolute_path: String::from("/foo/bar"),
        }
    }

    pub fn view() -> View {
        View {
            type_: type_(),
            array: vec![],
            forest: FileForest::new(),
        }
    }
}
