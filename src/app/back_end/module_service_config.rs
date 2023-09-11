use super::{
    users_module::api::users_service_config::user_api_service,
    admin_module::api::admin_service_config::admin_api_service,
    auth_module::api::auth_service_config::auth_api_service
};

use actix_web::{web, web::ServiceConfig};

pub fn back_end_module_service_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v0")
            .configure(user_api_service)
            .configure(auth_api_service)
    ).service(
        web::scope("")
            .configure(admin_api_service)
    );
}
