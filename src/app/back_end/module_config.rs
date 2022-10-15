use super::{
    users_module::api::users_config::user_api,
    admin_module::api::admin_config::admin_api,
    auth_module::api::auth_config::auth_api
};

use actix_web::{web, web::ServiceConfig};

pub fn back_end_module_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v0")
            .configure(user_api)
            .configure(auth_api)
    ).service(
        web::scope("")
            .configure(admin_api)
    );
}
