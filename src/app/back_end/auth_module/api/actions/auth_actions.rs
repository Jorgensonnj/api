
use reqwest::{Client, Error};
use std::collections::HashMap;

pub async fn login_action(address: &String) -> Result<HashMap<String, String>, Error> {

    let map = Client::new()
        .post(format!("{}/api/v0/logout", address))
        .send()
        .await
        .map_err(|error| {
            tracing::error!("failed to make request: {:?}", error);
            error
        })?
        .json::<HashMap<String, String>>()
        .await;

    map
}

