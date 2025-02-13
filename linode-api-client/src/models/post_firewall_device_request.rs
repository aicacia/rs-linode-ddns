/*
 * Akamai: Linode API
 *
 * Add a Cloud Computing instance so you can build, release, and scale applications faster with virtual machines. 
 *
 * The version of the OpenAPI document: 4.193.0
 * Contact: jperez@linode.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostFirewallDeviceRequest {
    /// The entity's ID.
    #[serde(rename = "id")]
    pub id: i32,
    /// __Read-only__ The entity's label.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The entity's type.
    #[serde(rename = "type")]
    pub r#type: TypeEnum,
    /// __Read-only__ The API URL path you can use to access this entity.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl PostFirewallDeviceRequest {
    pub fn new(id: i32, r#type: TypeEnum) -> PostFirewallDeviceRequest {
        PostFirewallDeviceRequest {
            id,
            label: None,
            r#type,
            url: None,
        }
    }
}
/// The entity's type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "linode")]
    Linode,
    #[serde(rename = "nodebalancer")]
    Nodebalancer,
}

impl Default for TypeEnum {
    fn default() -> TypeEnum {
        Self::Linode
    }
}

