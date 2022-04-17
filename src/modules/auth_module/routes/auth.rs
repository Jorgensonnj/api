use actix_web::{HttpRequest, HttpResponse, Responder};
use tracing::instrument;
use serde_json::Value;

#[instrument]
pub async fn login(_req: HttpRequest) -> impl Responder {
    let value: Value =
        serde_json::from_str(r#"{ "test" : "login"}"#)
            .unwrap();

    HttpResponse::Ok().json(value)
}

#[instrument]
pub async fn logout(_req: HttpRequest) -> impl Responder {
    let value: Value =
        serde_json::from_str(r#"{ "test" : "logout"}"#)
            .unwrap();

    HttpResponse::Ok().json(value)
}

#[instrument]
pub async fn refresh(_req: HttpRequest) -> impl Responder {
    let value: Value =
        serde_json::from_str(r#"{ "test" : "refresh"}"#)
            .unwrap();

    HttpResponse::Ok().json(value)
}
