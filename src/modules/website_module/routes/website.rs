use actix_web::{HttpRequest, Responder};
use actix_files::NamedFile;
use tracing::instrument;

// /index
#[instrument]
pub async fn index(_req: HttpRequest) -> impl Responder {

    // Start at the root directory
    let mut path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    // File path
    path.push_str("/src/modules/website_module/pages/index.html");

    NamedFile::open_async(path).await
}
