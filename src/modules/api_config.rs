use crate::modules::{admin::api::admin_config::admin_api, users::api::users_config::user_api};
use actix_web::{web, web::ServiceConfig};

pub fn api(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(admin_api)
            .configure(user_api)
    );
}
