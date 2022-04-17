use crate::modules::auth_module::routes::auth::*;
use actix_web::{web, web::ServiceConfig};

pub fn auth(cfg: &mut ServiceConfig) { 
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

