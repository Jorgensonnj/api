use actix_web::{HttpRequest, HttpResponse, Responder};
use serde_json::Value;

pub async fn status(_req: HttpRequest) -> impl Responder {
    let value: Value =
        serde_json::from_str(r#"{"status": "Alive"}"#)
            .unwrap();

    HttpResponse::Ok().json(value)
}
