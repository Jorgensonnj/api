use actix_web::{HttpRequest, Responder};
use actix_files::NamedFile;
use tracing::instrument;

// /index
#[instrument]
pub async fn index(_req: HttpRequest) -> impl Responder {
    NamedFile::open_async("./index.html").await
}
