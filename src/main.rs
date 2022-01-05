use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::result::Result;

const PRIORITY_NUMBER: usize = 444;
const RESOURCE_URL: &str =
  "https://management.azure.com/subscriptions/152b9fab-23f1-4afd-9048-dd45885ab0c4/resourceGroups/Ability-Sandbox/providers/Microsoft.Web/sites/ability-sandbox-proxy/config/web?api-version=2018-11-01";

fn main() -> Result<(), Box<dyn Error>> {
    let error_msg =
        "1 or 0 should be provided, where 1 will unlock API Playground and 0 will lock it";
    let param = std::env::args().skip(1).next().expect(error_msg);

    match param.as_str() {
        "0" => lock(),
        "1" => unlock(),
        _ => {
            panic!("{}", error_msg)
        }
    }?;

    Ok(())
}

fn unlock() -> Result<(), Box<dyn Error>> {
    let access_token = AccessToken::get_token()?;

    let mut restrictions = RestrictionsConfig::get_from_azure(&access_token)?;
    if let Some(_) = restrictions
        .restrictions
        .iter()
        .find(|n| n.priority == PRIORITY_NUMBER)
    {
        println!("API Playground is already unlocked");
        return Ok(());
    }

    restrictions.add(IpSecurityRestriction::new(
        String::from("0.0.0.0/0"),
        String::from("Allow"),
        String::from("Default"),
        PRIORITY_NUMBER,
        String::from("Temp Allow"),
    ));

    restrictions.patch_in_azure(
        &access_token, 
        "/subscriptions/152b9fab-23f1-4afd-9048-dd45885ab0c4/resourceGroups/Ability-Sandbox/providers/Microsoft.Web/sites/ability-sandbox-proxy/config/web", 
        "ability-sandbox-proxy",
        "Microsoft.Web/sites/config",
        "North Europe")?;

    Ok(())
}

fn lock() -> Result<(), Box<dyn Error>> {
    let access_token = AccessToken::get_token()?;

    let mut restrictions = RestrictionsConfig::get_from_azure(&access_token)?;

    if let None = restrictions
        .restrictions
        .iter()
        .find(|n| n.priority == PRIORITY_NUMBER)
    {
        println!("API Playground is already locked");
        return Ok(());
    }

    match restrictions.restrictions.iter().position(|n| n.priority == PRIORITY_NUMBER) {
        Some(index) => restrictions.restrictions.remove(index),
        None => {
            println!("API Playground is already locked");
            return Ok(());
        }
    };

    restrictions.patch_in_azure(
        &access_token, 
        "/subscriptions/152b9fab-23f1-4afd-9048-dd45885ab0c4/resourceGroups/Ability-Sandbox/providers/Microsoft.Web/sites/ability-sandbox-proxy/config/web", 
        "ability-sandbox-proxy",
        "Microsoft.Web/sites/config",
        "North Europe")?;

    Ok(())
}

#[derive(Debug)]
struct AccessToken {
    token: String,
}

impl AccessToken {
    pub fn get_token() -> Result<AccessToken, Box<dyn Error>> {
        const CLIENT_ID: &str = "fa8177f4-d1c9-4d2e-9cf3-66f0625cd693";
        const CLIENT_SECRET: &str = "r2U7Q~HGnYHx0b2hTTCJISKOGU-AHRvVxx~F0";

        let payload = AccessTokenRequest::new(String::from(CLIENT_ID), String::from(CLIENT_SECRET))
            .as_hashmap();

        let client = reqwest::blocking::Client::new();
        let response: AccessTokenResponse = client
            .post("https://login.microsoftonline.com/372ee9e0-9ce0-4033-a64a-c07073a91ecd/oauth2/token")
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

#[derive(Debug)]
struct RestrictionsConfig {
    restrictions: Vec<IpSecurityRestriction>,
}

impl RestrictionsConfig {
    pub fn get_from_azure(
        access_token: &AccessToken,
    ) -> Result<RestrictionsConfig, Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();
        let response: IpSecurityRestrictionsResponse = client
            .get(RESOURCE_URL)
            .header("Authorization", access_token.as_header())
            .send()?
            .json()?;

        Ok(RestrictionsConfig {
            restrictions: response.properties.ipSecurityRestrictions,
        })
    }

    pub fn add(&mut self, restriction: IpSecurityRestriction) {
        self.restrictions.push(restriction);
    }

    pub fn patch_in_azure(
        &self,
        access_token: &AccessToken,
        resource_id: &str,
        resource_name: &str,
        resource_type: &str,
        resource_location: &str,
    ) -> Result<(), Box<dyn Error>> {
        let request = IpRestrictionsPatchRequest {
            id: String::from(resource_id),
            name: String::from(resource_name),
            r#type: String::from(resource_type),
            location: String::from(resource_location),
            properties: IpSecurityRestrictionsPropertiesRequest {
                ipSecurityRestrictions: self.restrictions.as_slice(),
            },
        };
        let request = serde_json::to_string(&request)?;

        let client = reqwest::blocking::Client::new();
        let _ = client
            .patch(RESOURCE_URL)
            .body(request)
            .header("Authorization", access_token.as_header())
            .header("Content-Type", "application/json")
            .send()?;

        Ok(())
    }
}

#[derive(Serialize, Debug)]
struct IpRestrictionsPatchRequest<'a> {
    id: String,
    name: String,
    r#type: String,
    location: String,
    properties: IpSecurityRestrictionsPropertiesRequest<'a>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct IpSecurityRestrictionsPropertiesRequest<'a> {
    ipSecurityRestrictions: &'a [IpSecurityRestriction],
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct IpSecurityRestrictionsPropertiesResponse {
    ipSecurityRestrictions: Vec<IpSecurityRestriction>,
}

#[derive(Deserialize, Debug)]
struct IpSecurityRestrictionsResponse {
    properties: IpSecurityRestrictionsPropertiesResponse,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct IpSecurityRestriction {
    ipAddress: String,
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
    priority: usize,
    name: String,
}

impl IpSecurityRestriction {
    fn new(
        ip_address: String,
        action: String,
        tag: String,
        priority: usize,
        name: String,
    ) -> IpSecurityRestriction {
        IpSecurityRestriction {
            ipAddress: ip_address,
            action,
            tag: Some(tag),
            priority,
            name,
        }
    }
}
