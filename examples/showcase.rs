use actix_web::web;
use once_cell::sync;
use std::collections;
use std::io;

macro_rules! best_media_type_guess {
    ($relative_path:literal, $absolute_path:literal) => {
        once_cell::sync::Lazy::new(|| {
            let media_type = mime_guess::from_path($relative_path).first_or_octet_stream();
            String::from(media_type.essence_str())
        })
    };
}

#[iftree::include_file_tree(
    "
paths = '''
**
!.*
'''

base_folder = 'examples/assets'

[field_templates]
media_type = 'best_media_type_guess!'
"
)]
pub struct Asset {
    relative_path: &'static str,
    media_type: sync::Lazy<String>,
    content: &'static str,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .data(map_url_path_to_asset())
            .route("/{_:.*}", web::get().to(get_asset))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn map_url_path_to_asset() -> collections::HashMap<String, &'static Asset> {
    ASSETS
        .iter()
        .map(|asset| {
            let url_path = std::path::Path::new(asset.relative_path)
                .with_extension("")
                .to_string_lossy()
                .into_owned();
            (url_path, asset)
        })
        .collect()
}

async fn get_asset(
    assets: web::Data<collections::HashMap<String, &'static Asset>>,
    path: web::Path<String>,
) -> impl actix_web::Responder {
    match assets.get(&path.into_inner()) {
        None => actix_web::HttpResponse::NotFound().finish(),
        Some(asset) => actix_web::HttpResponse::Ok()
            .content_type(&*asset.media_type)
            .body(asset.content),
    }
}
