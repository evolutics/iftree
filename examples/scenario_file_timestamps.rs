use std::fs;
use std::sync;
use std::time;

macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        Asset {
            creation_time: sync::LazyLock::new(|| {
                fs::metadata($absolute_path)
                    .and_then(|metadata| metadata.created())
                    .ok()
            }),
            last_access_time: sync::LazyLock::new(|| {
                fs::metadata($absolute_path)
                    .and_then(|metadata| metadata.accessed())
                    .ok()
            }),
            last_modification_time: sync::LazyLock::new(|| {
                fs::metadata($absolute_path)
                    .and_then(|metadata| metadata.modified())
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
    creation_time: sync::LazyLock<Option<time::SystemTime>>,
    last_access_time: sync::LazyLock<Option<time::SystemTime>>,
    last_modification_time: sync::LazyLock<Option<time::SystemTime>>,
}

fn main() {
    use base::examples::assets;

    let creation_time = *assets::CREDITS_MD.creation_time;
    println!("Creation time: {creation_time:?}");

    let last_access_time = *assets::CREDITS_MD.last_access_time;
    println!("Last access time: {last_access_time:?}");

    let last_modification_time = *assets::CREDITS_MD.last_modification_time;
    println!("Last modification time: {last_modification_time:?}");
}
