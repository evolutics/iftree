macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        include_str!($absolute_path)
    };
}

#[iftree::include_file_tree(
    "
paths = '/examples/assets/**'
template.initializer = 'initialize'
debug = true
"
)]
pub type Asset = &'static str;

pub fn main() {
    eprintln!("Debug information:\n{}", DEBUG);
}
