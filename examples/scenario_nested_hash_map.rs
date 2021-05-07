use std::collections;
use std::ops;

pub enum Tree {
    File(Asset),
    Folder(collections::HashMap<&'static str, Tree>),
}

macro_rules! visit_base {
    ($length:literal, $($contents:expr)*) => {
        pub static ASSETS: once_cell::sync::Lazy<Tree> =
            once_cell::sync::Lazy::new(|| visit_folder!("", a, $($contents)*).1);
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
        ASSETS["examples"]["assets"]["credits.md"].unwrap().contents,
        "Boo Far\n",
    );
}

impl ops::Index<&str> for Tree {
    type Output = Tree;

    fn index(&self, name: &str) -> &Self::Output {
        match self {
            Tree::File(_) => panic!(),
            Tree::Folder(assets) => &assets[name],
        }
    }
}

impl Tree {
    fn unwrap(&self) -> &Asset {
        match self {
            Tree::File(asset) => asset,
            Tree::Folder(_) => panic!(),
        }
    }
}
