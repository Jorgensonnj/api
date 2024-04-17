use actix_web::{HttpRequest, HttpResponse, Responder, web::{Data, Json}};
use sqlx::{Pool, Postgres, Error};
use tracing::instrument;
use serde_json::Value;

use crate::config::Settings;
use super::super::{
    actions::auth_actions::*,
    super::super::shared_module::api::models::*
};
use std::collections::HashMap;

//use std::collections::HashMap;
//use reqwest::Client;

#[instrument]
pub async fn login(
    _req: HttpRequest, data_config: Data<Settings>,
    data_pool: Data<Result<Pool<Postgres>, Error>>,
    payload: Json<JsonUser>
) -> impl Responder {

    let address = match &data_config.get_ref().modules {
        Some(module_settings) => {
            match module_settings.get(&"auth_module".to_string()) {
                Some(auth_module_settings) => auth_module_settings.address_string(),
                None => "http://localhost".to_string()
            }
        }
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
