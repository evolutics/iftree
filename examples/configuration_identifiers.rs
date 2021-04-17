#[iftree::include_file_tree(
    "
paths = '/examples/resources/**'
identifiers = false
"
)]
pub struct Resource;

pub fn main() {
    assert_eq!(ASSETS.len(), 6);

    // No module `base` is generated.
    // use base::*;
}
