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

/// GetTickets200ResponseDataInnerEntity : __Read-only__ The ticket was opened for this entity. An entity represents a specific object you've created, such as a Linode or a Managed Database.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetTickets200ResponseDataInnerEntity {
    /// __Read-only__ The unique ID for this ticket's entity. Empty if the targeted entity doesn't use an `id`.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// __Read-only__ The current label of this entity.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// __Read-only__ The type of entity.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TypeEnum>,
    /// __Read-only__ The URL where you can access the `entity`. If this is a relative URL, it's relative to the domain for the entity.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl GetTickets200ResponseDataInnerEntity {
    /// __Read-only__ The ticket was opened for this entity. An entity represents a specific object you've created, such as a Linode or a Managed Database.
    pub fn new() -> GetTickets200ResponseDataInnerEntity {
        GetTickets200ResponseDataInnerEntity {
            id: None,
            label: None,
            r#type: None,
            url: None,
        }
    }
}
/// __Read-only__ The type of entity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "database")]
    Database,
    #[serde(rename = "domain")]
    Domain,
    #[serde(rename = "firewall")]
    Firewall,
    #[serde(rename = "linode")]
    Linode,
    #[serde(rename = "lkecluster")]
    Lkecluster,
    #[serde(rename = "managed_service")]
    ManagedService,
    #[serde(rename = "nodebalancer")]
    Nodebalancer,
    #[serde(rename = "vlan")]
    Vlan,
    #[serde(rename = "volume")]
    Volume,
    #[serde(rename = "vpc")]
    Vpc,
}

impl Default for TypeEnum {
    fn default() -> TypeEnum {
        Self::Database
    }
}

