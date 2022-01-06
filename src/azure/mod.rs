pub mod http_payloads;

use crate::access_token::AccessToken;
use std::error::Error;

use self::http_payloads::{IpSecurityRestrictionsResponse, IpSecurityRestriction, IpRestrictionsPatchRequest, IpSecurityRestrictionsPropertiesRequest};

pub struct WebAppDetails<'a> {
    pub subscription_id: &'a str,
    pub rg_name: &'a str,
    pub web_app_name: &'a str,
    pub location: &'a str,
}

#[derive(Debug)]
pub struct WebAppRestrictionsManager {
    pub restrictions: Vec<IpSecurityRestriction>,
}

impl WebAppRestrictionsManager {
    fn get_resource_url(&self, web_app_details: &WebAppDetails) -> String {
        String::from(format!(
            "https://management.azure.com{}?api-version=2018-11-01",
            self.get_resource_id(web_app_details)
        ))
    }

    fn get_resource_id(&self, web_app_details: &WebAppDetails) -> String {
        String::from(format!(
            "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites/{}/config/web",
            web_app_details.subscription_id, web_app_details.rg_name, web_app_details.web_app_name
        ))
    }

    pub fn new() -> WebAppRestrictionsManager {
        WebAppRestrictionsManager {
            restrictions: vec![],
        }
    }

    pub fn get_from_azure(
        &self,
        access_token: &AccessToken,
        web_app_details: &WebAppDetails,
    ) -> Result<WebAppRestrictionsManager, Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();
        let response: IpSecurityRestrictionsResponse = client
            .get(self.get_resource_url(web_app_details))
            .header("Authorization", access_token.as_header())
            .send()?
            .json()?;

        Ok(WebAppRestrictionsManager {
            restrictions: response.properties.ipSecurityRestrictions,
        })
    }

    pub fn add(&mut self, restriction: IpSecurityRestriction) {
        self.restrictions.push(restriction);
    }

    pub fn patch_in_azure(
        &self,
        access_token: &AccessToken,
        web_app_details: &WebAppDetails,
    ) -> Result<(), Box<dyn Error>> {
        let request = IpRestrictionsPatchRequest {
            id: self.get_resource_id(&web_app_details),
            name: String::from(web_app_details.web_app_name),
            r#type: String::from("Microsoft.Web/sites/config"),
            location: String::from(web_app_details.location),
            properties: IpSecurityRestrictionsPropertiesRequest {
                ipSecurityRestrictions: self.restrictions.as_slice(),
            },
        };
        let request = serde_json::to_string(&request)?;

        let client = reqwest::blocking::Client::new();
        let _ = client
            .patch(self.get_resource_url(web_app_details))
            .body(request)
            .header("Authorization", access_token.as_header())
            .header("Content-Type", "application/json")
            .send()?;

        Ok(())
    }
}