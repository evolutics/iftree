use sha2::Digest;
use std::sync;

macro_rules! initialize {
    ($relative_path:literal, $absolute_path:literal) => {
        Asset {
            sha_256_hash: sync::LazyLock::new(|| {
                sha2::Sha256::new()
                    .chain_update(include_bytes!($absolute_path))
                    .finalize()
                    .into()
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
    sha_256_hash: sync::LazyLock<[u8; 32]>,
}

fn main() {
    use base::examples::assets;

    assert_eq!(
        (*assets::CREDITS_MD.sha_256_hash)[..8],
        *b"\x41\xDE\xCC\x43\x49\xAB\x68\xBF",
    );
}
