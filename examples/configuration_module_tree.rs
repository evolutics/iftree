#[iftree::include_file_tree(
    "
resource_paths = '/examples/resources/**'
module_tree = false
"
)]
pub struct Resource;

pub fn main() {
    assert_eq!(ARRAY.len(), 6);

    // No module `base` is generated.
    // use base::*;
}
