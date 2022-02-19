use crate::modules::admin_module::api::routes::*;
use actix_web::{web, web::ServiceConfig};

pub fn admin_api(cfg: &mut ServiceConfig) { 
    cfg.route("/v1/status", web::get().to(status));
}

