use std::path;
use std::vec;

pub fn main() -> vec::Vec<path::PathBuf> {
    vec![
        path::PathBuf::from("configuration/menu.json"),
        path::PathBuf::from("configuration/translations.csv"),
        path::PathBuf::from("credits.md"),
        path::PathBuf::from("world/levels/tutorial.json"),
        path::PathBuf::from("world/physical_constants.json"),
    ]
}
