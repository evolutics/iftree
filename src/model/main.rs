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
    pub initializer: Option<String>,
    pub identifiers: bool,
    pub debug: bool,
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct Type<T> {
    pub name: syn::Ident,
    pub structure: TypeStructure<T>,
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum TypeStructure<T> {
    Unit,
    TypeAlias(T),
    NamedFields(vec::Vec<(String, T)>),
    TupleFields(vec::Vec<T>),
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct Path {
    pub relative: RelativePath,
    pub absolute: String,
}

#[derive(Clone, cmp::Eq, cmp::Ord, cmp::PartialEq, cmp::PartialOrd, Debug)]
pub struct RelativePath(pub String);

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct View {
    pub type_: syn::Ident,
    pub visitors: vec::Vec<Visitor>,
    pub count: usize,
    pub forest: FileForest,
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum Visitor {
    Array(Initializer),
    Identifiers,
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum Initializer {
    Default(TypeStructure<Populator>),
    Macro(String),
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum Populator {
    ContentsBytes,
    ContentsStr,
    GetBytes,
    GetStr,
    RelativePath,
}

pub type FileForest = collections::BTreeMap<String, FileTree>;

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum FileTree {
    File(File),
    Folder(Folder),
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct File {
    pub identifier: syn::Ident,
    pub index: usize,
    pub relative_path: RelativePath,
    pub absolute_path: String,
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct Folder {
    pub identifier: syn::Ident,
    pub forest: FileForest,
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum Error {
    EnvironmentVariable { name: String, source: env::VarError },
    Ignore(IgnoreError),
    NoInitializer,
    NonstandardField { field: String },
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
            initializer: None,
            identifiers: false,
            debug: false,
        }
    }

    pub fn type_<T>() -> Type<T> {
        Type {
            name: quote::format_ident!("Foo"),
            structure: type_structure(),
        }
    }

    pub fn type_structure<T>() -> TypeStructure<T> {
        TypeStructure::Unit
    }

    pub fn view() -> View {
        View {
            type_: quote::format_ident!("Foo"),
            visitors: vec![],
            count: 0,
            forest: FileForest::new(),
        }
    }

    pub fn file() -> File {
        File {
            identifier: quote::format_ident!("BAR"),
            index: 123,
            relative_path: RelativePath::from("bar"),
            absolute_path: String::from("/foo/bar"),
        }
    }
}
