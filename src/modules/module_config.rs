use crate::modules::{
    admin_module::api::admin_config::admin_api,
    users_module::api::users_config::user_api,
    website_module::website_config::website
};
use actix_web::{web, web::ServiceConfig};

pub fn api_module(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(admin_api)
            .configure(user_api)
    );
}

pub fn website_module(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/")
            .configure(website)
    );
}
