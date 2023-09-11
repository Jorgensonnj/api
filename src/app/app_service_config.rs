use super::{
    back_end::module_service_config::back_end_module_service_config,
    front_end::module_service_config::front_end_module_service_config
};


use actix_web::{web, web::ServiceConfig};

pub fn back_end_service_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(back_end_module_service_config)
    );
}

pub fn front_end_service_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(front_end_module_service_config)
    );
}


