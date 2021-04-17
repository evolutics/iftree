#[iftree::include_file_tree(
    "
paths = '/examples/assets/**'
debug = true

[field_templates]
_ = 'content'
"
)]
pub type Asset = &'static str;

pub fn main() {
    eprintln!("Debug information:\n{}", DEBUG);
}
