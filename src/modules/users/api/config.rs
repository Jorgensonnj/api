use crate::modules::users::api::routes::*;
use actix_web::{web, web::ServiceConfig};

pub fn user_api(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(
                web::resource("")
                    .route(web::post().to(create_user))
                    .route(web::get().to(read_users))
            )
            .service(
                web::resource("/{user_id}")
                    .route(web::get().to(read_user))
                    .route(web::put().to(update_user))
                    .route(web::delete().to(delete_user))
            )
    );
}
