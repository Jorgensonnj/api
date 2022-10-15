use actix_web::{web, web::ServiceConfig};
use super::routes::auth::*;

pub fn auth_api(cfg: &mut ServiceConfig) { 
    cfg.service(
        web::resource("/login")
            .route(web::get().to(login))
    )
    .service(
        web::resource("/logout")
            .route(web::get().to(logout))
    )
    .service(web::resource("/refresh")
            .route(web::get().to(refresh))
    );
}

