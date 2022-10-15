use super::{
    back_end::module_config::back_end_module_config,
    front_end::module_config::front_end_module_config
};


use actix_web::{web, web::ServiceConfig};

pub fn back_end_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(back_end_module_config)
    );
}

pub fn front_end_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(front_end_module_config)
    );
}


