use crate::modules::website::routes::index;
//use actix_files::File;
use actix_web::{web, web::ServiceConfig};

pub fn website(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(index)
        )
    );
}
