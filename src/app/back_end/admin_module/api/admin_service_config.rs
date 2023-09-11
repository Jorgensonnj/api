use super::routes::*;
use actix_web::{web, web::ServiceConfig};

pub fn admin_api_service(cfg: &mut ServiceConfig) {
    cfg.route("/status", web::get().to(status));
}

