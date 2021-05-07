use std::cmp;
use std::path;
use std::vec;

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Configuration {
    pub paths: String,
    pub base_folder: Option<path::PathBuf>,
    pub root_folder_variable: Option<String>,
    pub template: Option<Template>,
    pub debug: Option<bool>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum Template {
    Default {
        initializer: Option<Path>,
        identifiers: Option<bool>,
    },
    Visitors(vec::Vec<CustomVisitor>),
}

#[derive(cmp::PartialEq, Debug)]
pub struct Path(pub syn::Path);

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomVisitor {
    pub visit_base: Option<Path>,
    pub visit_folder: Option<Path>,
    pub visit_file: Path,
}
