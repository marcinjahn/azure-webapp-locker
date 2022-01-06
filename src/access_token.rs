use std::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Debug)]
pub struct AccessToken {
    token: String,
}

impl AccessToken {
    pub fn get_token(
        client_id: &str,
        client_secret: &str,
        tenant_id: &str,
    ) -> Result<AccessToken, Box<dyn Error>> {
        let payload = AccessTokenRequest::new(String::from(client_id), String::from(client_secret))
            .as_hashmap();

        let client = reqwest::blocking::Client::new();
        let response: AccessTokenResponse = client
            .post(format!(
                "https://login.microsoftonline.com/{}/oauth2/token",
                tenant_id
            ))
            .form(&payload)
            .send()?
            .json()?;

        Ok(AccessToken {
            token: response.access_token,
        })
    }

    pub fn as_header(&self) -> String {
        format!("Bearer {}", self.token)
    }
}

#[derive(Serialize)]
struct AccessTokenRequest {
    resource: String,
    client_id: String,
    client_secret: String,
    grant_type: String,
}

impl AccessTokenRequest {
    pub fn new(client_id: String, client_secret: String) -> AccessTokenRequest {
        AccessTokenRequest {
            client_id,
            client_secret,
            resource: String::from("https://management.azure.com/"),
            grant_type: String::from("client_credentials"),
        }
    }

    pub fn as_hashmap(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert(String::from("resource"), self.resource.clone());
        map.insert(String::from("client_id"), self.client_id.clone());
        map.insert(String::from("client_secret"), self.client_secret.clone());
        map.insert(String::from("grant_type"), self.grant_type.clone());

        map
    }
}

#[derive(Deserialize)]
struct AccessTokenResponse {
    access_token: String,
}