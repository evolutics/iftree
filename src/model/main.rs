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
    pub base_folder_environment_variable: String,
}

#[derive(Clone, cmp::PartialEq, Debug)]
pub struct ResourceType {
    pub identifier: String,
    pub structure: Fields<()>,
}

#[derive(Clone, cmp::Eq, cmp::Ord, cmp::PartialEq, cmp::PartialOrd, Debug)]
pub enum Fields<T> {
    TypeAlias(T),
    #[allow(dead_code)]
    NamedFields(collections::BTreeMap<String, T>),
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
    pub fields: Fields<proc_macro2::TokenStream>,
}

#[cfg(test)]
pub mod stubs {
    use super::*;

    pub fn configuration() -> Configuration {
        Configuration {
            resource_paths: String::from("!*"),
            resolve_name_collisions: false,
            base_folder_environment_variable: String::from("FOO"),
        }
    }

    pub fn resource_type() -> ResourceType {
        ResourceType {
            identifier: String::from("foo"),
            structure: fields(()),
        }
    }

    pub fn fields<T>(value: T) -> Fields<T> {
        Fields::TypeAlias(value)
    }

    pub fn file() -> File {
        File {
            relative_path: path::PathBuf::from("bar"),
            fields: Fields::TypeAlias(quote::quote! {
                include_str!("/foo/bar")
            }),
        }
    }
}
