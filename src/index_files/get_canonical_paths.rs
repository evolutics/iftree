use crate::model;
use std::path;
use std::vec;

pub fn main() -> model::Result<vec::Vec<path::PathBuf>> {
    Ok(vec![
        path::PathBuf::from("configuration/menu.json"),
        path::PathBuf::from("configuration/translations.csv"),
        path::PathBuf::from("credits.md"),
        path::PathBuf::from("world/levels/tutorial.json"),
        path::PathBuf::from("world/physical_constants.json"),
    ])
}
