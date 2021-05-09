use once_cell::sync;
use std::collections;

enum Tree {
    File(Asset),
    Folder(collections::HashMap<&'static str, Tree>),
}

macro_rules! visit_base {
    ($length:literal, $($contents:expr)*) => {
        static ASSETS: sync::Lazy<Tree> =
            sync::Lazy::new(|| Tree::Folder(vec![$($contents,)*].into_iter().collect()));
    };
}

macro_rules! visit_folder {
    ($name:literal, $id:ident, $($contents:expr)*) => {
        (
            $name,
            Tree::Folder(vec![$($contents,)*].into_iter().collect()),
        )
    };
}

macro_rules! visit_file {
    ($name:literal, $id:ident, $index:literal, $relative_path:literal, $absolute_path:literal) => {
        (
            $name,
            Tree::File(Asset {
                contents: include_str!($absolute_path),
            }),
        )
    };
}

#[iftree::include_file_tree(
    "
paths = '/examples/assets/**'

[[template]]
visit_base = 'visit_base'
visit_folder = 'visit_folder'
visit_file = 'visit_file'
"
)]
pub struct Asset {
    contents: &'static str,
}

pub fn main() {
    assert_eq!(
        get_asset(&ASSETS, &["examples", "assets", "credits.md"])
            .unwrap()
            .contents,
        "Boo Far\n",
    );

    assert!(get_asset(&ASSETS, &["examples", "assets", "seed.json"]).is_none());
}

fn get_asset<'a>(tree: &'a Tree, path: &[&'a str]) -> Option<&'a Asset> {
    match (tree, path.split_first()) {
        (Tree::File(asset), None) => Some(asset),
        (Tree::Folder(forest), Some((name, subpath))) => forest
            .get(name)
            .and_then(|subtree| get_asset(subtree, subpath)),
        _ => None,
    }
}
