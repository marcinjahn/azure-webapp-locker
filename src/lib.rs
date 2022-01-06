use access_token::AccessToken;
use azure::{WebAppRestrictionsManager, WebAppDetails, http_payloads::IpSecurityRestriction};
use serde::Deserialize;
use std::error::Error;

mod azure;
mod access_token;


#[derive(Deserialize, Debug)]
pub struct AppConfig {
    subscription_id: String,
    rg_name: String,
    web_app_name: String,
    web_app_location: String,
    client_id: String,
    client_secret: String,
    tenant_id: String,
    rule_priority_number: usize,
}

pub fn unlock(config: AppConfig) -> Result<(), Box<dyn Error>> {
    println!("Attempting to unlock the '{}' Web App", &config.web_app_name);

    let access_token =
        AccessToken::get_token(&config.client_id, &config.client_secret, &config.tenant_id)?;

    let manager = WebAppRestrictionsManager::new();
    let web_app_details = WebAppDetails {
        subscription_id: &config.subscription_id,
        rg_name: &config.rg_name,
        web_app_name: &config.web_app_name,
        location: &config.web_app_location,
    };
    let mut restrictions = manager.get_from_azure(&access_token, &web_app_details)?;

    if restrictions
        .restrictions
        .iter()
        .any(|n| n.priority == config.rule_priority_number)
    {
        println!("The '{}' Web App is already unlocked", &config.web_app_name);
        return Ok(());
    }

    restrictions.add(IpSecurityRestriction::new(
        String::from("0.0.0.0/0"),
        String::from("Allow"),
        String::from("Default"),
        config.rule_priority_number,
        String::from("Temp Allow"),
    ));

    restrictions.patch_in_azure(&access_token, &web_app_details)?;

    println!("The '{}' Web App is now unlocked", &config.web_app_name);

    Ok(())
}

pub fn lock(config: AppConfig) -> Result<(), Box<dyn Error>> {
    println!("Attempting to lock the '{}' Web App", &config.web_app_name);

    let access_token =
        AccessToken::get_token(&config.client_id, &config.client_secret, &config.tenant_id)?;

    let manager = WebAppRestrictionsManager::new();
    let web_app_details = WebAppDetails {
        subscription_id: &config.subscription_id,
        rg_name: &config.rg_name,
        web_app_name: &config.web_app_name,
        location: &config.web_app_location,
    };
    let mut restrictions = manager.get_from_azure(&access_token, &web_app_details)?;

    match restrictions
        .restrictions
        .iter()
        .position(|n| n.priority == config.rule_priority_number)
    {
        Some(index) => restrictions.restrictions.remove(index),
        None => {
            println!("The '{}' Web App is already locked", &config.web_app_name);
            return Ok(());
        }
    };

    restrictions.patch_in_azure(&access_token, &web_app_details)?;

    println!("The '{}' Web App is now locked", &config.web_app_name);

    Ok(())
}