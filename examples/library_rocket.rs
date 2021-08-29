use std::path;

#[iftree::include_file_tree(
    "
paths = '**'
base_folder = 'examples/assets'
"
)]
pub struct Asset {
    relative_path: &'static str,
    contents_str: &'static str,
}

#[rocket::launch]
fn launch() -> _ {
    rocket::build().mount("/", rocket::routes![get_asset])
}

#[rocket::get("/<path..>")]
fn get_asset(path: path::PathBuf) -> Option<&'static str> {
    let path = path
        .iter()
        .map(|component| component.to_string_lossy())
        .collect::<Vec<_>>()
        .join("/");

    // For a more efficient lookup, see the `scenario_hash_map` example.
    ASSETS
        .iter()
        .position(|asset| asset.relative_path == path)
        .map(|index| ASSETS[index].contents_str)
}
