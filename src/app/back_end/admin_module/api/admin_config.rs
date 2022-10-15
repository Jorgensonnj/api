use super::routes::*;
use actix_web::{web, web::ServiceConfig};

pub fn admin_api(cfg: &mut ServiceConfig) {
    cfg.route("/status", web::get().to(status));
}

