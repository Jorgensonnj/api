use super::website_module::website_config::website;

use actix_web::{web, web::ServiceConfig};

pub fn front_end_module_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(website)
    );
}
