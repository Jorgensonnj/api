use actix_web::{web, web::ServiceConfig};
use super::routes::auth_routes::*;

pub fn auth_api_service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/login")
            .route(web::post().to(login))
    )
    .service(
        web::resource("/logout")
            .route(web::post().to(logout))
    )
    .service(web::resource("/refresh")
            .route(web::post().to(refresh))
    );
}

