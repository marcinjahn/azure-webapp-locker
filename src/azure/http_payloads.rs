use serde::{Serialize, Deserialize};


#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct IpSecurityRestriction {
    pub ipAddress: String,
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub priority: usize,
    pub name: String,
}

impl IpSecurityRestriction {
    pub fn new(
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

#[derive(Serialize, Debug)]
pub struct IpRestrictionsPatchRequest<'a> {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) r#type: String,
    pub(crate) location: String,
    pub(crate) properties: IpSecurityRestrictionsPropertiesRequest<'a>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct IpSecurityRestrictionsPropertiesRequest<'a> {
    pub(crate) ipSecurityRestrictions: &'a [IpSecurityRestriction],
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct IpSecurityRestrictionsPropertiesResponse {
    pub(crate) ipSecurityRestrictions: Vec<IpSecurityRestriction>,
}

#[derive(Deserialize, Debug)]
pub struct IpSecurityRestrictionsResponse {
    pub(crate) properties: IpSecurityRestrictionsPropertiesResponse,
}