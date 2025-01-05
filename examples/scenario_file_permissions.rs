use std::fs;
use std::sync;

macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        Asset {
            permissions: sync::LazyLock::new(|| {
                fs::metadata($absolute_path)
                    .map(|metadata| metadata.permissions())
                    .ok()
            }),
        }
    };
}

#[iftree::include_file_tree(
    "
paths = '/examples/assets/**'
template.initializer = 'initialize'
"
)]
pub struct Asset {
    permissions: sync::LazyLock<Option<fs::Permissions>>,
}

fn main() {
    use base::examples::assets;

    let permissions = &*assets::CREDITS_MD.permissions;
    println!("Permissions: {permissions:?}");
}
