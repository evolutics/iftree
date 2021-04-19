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
    content: &'static str,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new().route("/{path:.*}", web::get().to(get_asset))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_asset(request: actix_web::HttpRequest) -> impl actix_web::Responder {
    let path = request.match_info().query("path");
    match ASSETS.binary_search_by_key(&path, |asset| asset.relative_path) {
        Err(_) => actix_web::HttpResponse::NotFound().finish(),
        Ok(index) => actix_web::HttpResponse::Ok().body(ASSETS[index].content),
    }
}
