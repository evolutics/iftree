use actix_web::web;
use std::io;

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

#[actix_web::main]
async fn main() -> io::Result<()> {
    actix_web::HttpServer::new(|| actix_web::App::new().route("/{_:.*}", web::get().to(get_asset)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn get_asset(path: web::Path<String>) -> impl actix_web::Responder {
    let path = path.into_inner();
    // For a more efficient lookup, see the `scenario_hash_map` example.
    match ASSETS.iter().position(|asset| asset.relative_path == path) {
        None => actix_web::HttpResponse::NotFound().finish(),
        Some(index) => actix_web::HttpResponse::Ok().body(ASSETS[index].contents_str),
    }
}
