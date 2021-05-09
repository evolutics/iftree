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

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/*path").get(get_asset);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn get_asset(request: tide::Request<()>) -> tide::Result {
    let path = request.param("path")?;
    Ok(
        match ASSETS.iter().position(|asset| asset.relative_path == path) {
            None => tide::Response::new(tide::StatusCode::NotFound),
            Some(index) => tide::Response::from(ASSETS[index].contents_str),
        },
    )
}
