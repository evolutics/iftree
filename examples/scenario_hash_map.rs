use std::collections;
use std::sync;

#[iftree::include_file_tree("paths = '/examples/assets/**'")]
pub struct Asset {
    relative_path: &'static str,
    contents_str: &'static str,
}

static ASSET_MAP: sync::LazyLock<collections::HashMap<&str, &Asset>> = sync::LazyLock::new(|| {
    ASSETS
        .iter()
        .map(|asset| (asset.relative_path, asset))
        .collect()
});

fn main() {
    assert_eq!(ASSET_MAP.len(), 6);

    let mut keys = ASSET_MAP.keys().collect::<Vec<_>>();
    keys.sort_unstable();
    assert_eq!(
        keys,
        vec![
            &"examples/assets/.env",
            &"examples/assets/configuration/menu.json",
            &"examples/assets/configuration/translations.csv",
            &"examples/assets/credits.md",
            &"examples/assets/world/levels/tutorial.json",
            &"examples/assets/world/physical_constants.json",
        ],
    );

    assert_eq!(
        ASSET_MAP
            .get("examples/assets/credits.md")
            .unwrap()
            .contents_str,
        "Boo Far\n",
    );

    assert!(ASSET_MAP.get("examples/assets/seed.json").is_none());
}
