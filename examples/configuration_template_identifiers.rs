#[iftree::include_file_tree(
    "
paths = '/examples/assets/**'
template.identifiers = false
"
)]
pub struct Asset;

fn main() {
    assert_eq!(ASSETS.len(), 6);

    // No module `base` is generated.
    // use base::*;
}
