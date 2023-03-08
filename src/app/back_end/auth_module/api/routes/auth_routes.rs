use actix_web::{HttpRequest, HttpResponse, Responder, web::{Data, Json}};
use sqlx::{Pool, Postgres, Error};
use tracing::instrument;
use serde_json::Value;

use crate::config::Settings;
use super::super::{
    actions::auth_actions::*,
    models::auth_models::*
};
use std::collections::HashMap;

//use std::collections::HashMap;
//use reqwest::Client;

#[instrument]
pub async fn login(
    _req: HttpRequest, data_config: Data<Settings>,
    data_pool: Data<Result<Pool<Postgres>, Error>>,
    payload: Json<AuthJsonUser>
) -> impl Responder {

    let module_setting = data_config.get_ref().modules.get(&"auth_module".to_string());
    let address = match module_setting {
        Some(auth_module) => auth_module.address_string(),
        None => "http://localhost".to_string()
    };

    let v = login_action(&address).await.unwrap_or(HashMap::new());

    println!("{:?}", v);

    let value: Value =
        serde_json::from_str(r#"{ "test" : "login"}"#)
            .unwrap();

    HttpResponse::Ok().json(value)
}

#[instrument]
pub async fn logout(_req: HttpRequest) -> impl Responder {
    let value: Value =
        serde_json::from_str(r#"{ "test" : "logout"}"#)
            .unwrap();

    HttpResponse::Ok().json(value)
}

#[instrument]
pub async fn refresh(_req: HttpRequest) -> impl Responder {
    let value: Value =
        serde_json::from_str(r#"{ "test" : "refresh"}"#)
            .unwrap();

    HttpResponse::Ok().json(value)
}
