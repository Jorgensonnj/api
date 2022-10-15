use super::routes::website::*;
use actix_files::Files;

use actix_web::{web, web::ServiceConfig};

pub fn website(cfg: &mut ServiceConfig) {

    let mut website_module_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    website_module_path.push_str("/src/app/front_end/website_module");

    let css_path = format!("{}{}", website_module_path, "/css");

    cfg.service(
        web::resource("/")
            .route(web::get().to(index)
        )
    )
    .service(
        Files::new("/css", css_path)
    );
}
