#[iftree::include_file_tree(
    "
resource_paths = '/examples/resources/**'
module_tree = false
"
)]
pub struct Resource;

pub fn main() {
    assert_eq!(ARRAY.len(), 6);

    #[allow(unused_imports)]
    use base::*;
}
