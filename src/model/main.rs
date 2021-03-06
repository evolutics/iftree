use std::cmp;
use std::collections;
use std::env;
use std::path;
use std::result;

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct Configuration {
    pub paths: String,
    pub base_folder: path::PathBuf,
    pub root_folder_variable: String,
    pub template: Template,
    pub debug: bool,
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum Template {
    Default {
        initializer: Option<syn::Path>,
        identifiers: bool,
    },
    Visitors(Vec<CustomVisitor>),
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct CustomVisitor {
    pub visit_base: Option<syn::Path>,
    pub visit_folder: Option<syn::Path>,
    pub visit_file: syn::Path,
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
    NamedFields(Vec<(syn::Ident, T)>),
    TupleFields(Vec<T>),
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct Path {
    pub relative: Vec<String>,
    pub absolute: String,
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct View {
    pub type_: syn::Ident,
    pub visitors: Vec<Visitor>,
    pub forest: Forest,
    pub debug: bool,
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum Visitor {
    Array(Initializer),
    Identifiers,
    Custom(CustomVisitor),
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum Initializer {
    Default(TypeStructure<Populator>),
    Macro(syn::Path),
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum Populator {
    ContentsBytes,
    ContentsStr,
    GetBytes,
    GetStr,
    RelativePath,
}

pub type Forest = collections::BTreeMap<String, Tree>;

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum Tree {
    File(File),
    Folder(Folder),
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct File {
    pub identifier: syn::Ident,
    pub index: usize,
    pub relative_path: String,
    pub absolute_path: String,
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct Folder {
    pub identifier: syn::Ident,
    pub forest: Forest,
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, cmp::PartialEq, Debug)]
pub enum Error {
    EnvironmentVariable {
        name: String,
        source: env::VarError,
    },
    Ignore(IgnoreError),
    NoInitializer,
    NonstandardField {
        field: syn::Ident,
        standard_fields: Vec<syn::Ident>,
    },
    PathInvalidUnicode(path::PathBuf),
    PathStripPrefix(path::StripPrefixError),
    UnexpectedEmptyRelativePath {
        absolute_path: path::PathBuf,
    },
    UnexpectedPathCollision(path::PathBuf),
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
            template: Template::Visitors(vec![]),
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

    pub fn path() -> Path {
        Path {
            relative: vec![String::from("bar")],
            absolute: String::from("/foo/bar"),
        }
    }

    pub fn view() -> View {
        View {
            type_: quote::format_ident!("Foo"),
            visitors: vec![],
            forest: Forest::new(),
            debug: false,
        }
    }

    pub fn file() -> File {
        File {
            identifier: quote::format_ident!("BAR"),
            index: 123,
            relative_path: String::from("bar"),
            absolute_path: String::from("/foo/bar"),
        }
    }

    pub fn folder() -> Folder {
        Folder {
            identifier: quote::format_ident!("foo"),
            forest: Forest::new(),
        }
    }
}
