use warp::Filter;
use warp::path;
use warp::reject;

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

#[tokio::main]
async fn main() {
    warp::serve(path::tail().and_then(get_asset))
        .run(([127, 0, 0, 1], 8080))
        .await;
}

async fn get_asset(path: path::Tail) -> Result<&'static str, reject::Rejection> {
    let path = path.as_str();
    // For a more efficient lookup, see the `scenario_hash_map` example.
    match ASSETS.iter().position(|asset| asset.relative_path == path) {
        None => Err(reject::not_found()),
        Some(index) => Ok(ASSETS[index].contents_str),
    }
}
