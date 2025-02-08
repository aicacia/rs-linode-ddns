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

/// GetFirewalls200ResponseDataInner : A resource that controls incoming and outgoing network traffic to a compute service. Only one enabled Firewall can be attached to a particular service at any given time. [Create a firewall device](https://techdocs.akamai.com/linode-api/reference/post-firewall-device) to assign a Firewall to a service. Currently, Firewalls can assigned to Linode compute instances and NodeBalancers.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetFirewalls200ResponseDataInner {
    /// __Filterable__, __Read-only__ When this Firewall was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// __Filterable__, __Read-only__ The Firewall's unique ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// __Filterable__ The Firewall's label, for display purposes only.  Firewall labels have the following constraints:    - Must begin and end with an alphanumeric character.   - May only consist of alphanumeric characters, hyphens (`-`), underscores (`_`) or periods (`.`).   - Cannot have two hyphens (`--`), underscores (`__`) or periods (`..`) in a row.   - Must be between 3 and 32 characters.   - Must be unique.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<models::GetFirewalls200ResponseDataInnerRules>,
    /// __Read-only__ The status of this Firewall.    - When a Firewall is first created its status is `enabled`.   - Run the [Update a firewall](https://techdocs.akamai.com/linode-api/reference/put-firewall) operation to set a Firewall's status to `enabled` or `disabled`.   - Run the [Delete a firewall](https://techdocs.akamai.com/linode-api/reference/delete-firewall) operation to delete a Firewall.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusEnum>,
    /// __Filterable__ An array of tags applied to this object. Tags are for organizational purposes only.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// __Filterable__, __Read-only__ When this Firewall was last updated.
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

impl GetFirewalls200ResponseDataInner {
    /// A resource that controls incoming and outgoing network traffic to a compute service. Only one enabled Firewall can be attached to a particular service at any given time. [Create a firewall device](https://techdocs.akamai.com/linode-api/reference/post-firewall-device) to assign a Firewall to a service. Currently, Firewalls can assigned to Linode compute instances and NodeBalancers.
    pub fn new() -> GetFirewalls200ResponseDataInner {
        GetFirewalls200ResponseDataInner {
            created: None,
            id: None,
            label: None,
            rules: None,
            status: None,
            tags: None,
            updated: None,
        }
    }
}
/// __Read-only__ The status of this Firewall.    - When a Firewall is first created its status is `enabled`.   - Run the [Update a firewall](https://techdocs.akamai.com/linode-api/reference/put-firewall) operation to set a Firewall's status to `enabled` or `disabled`.   - Run the [Delete a firewall](https://techdocs.akamai.com/linode-api/reference/delete-firewall) operation to delete a Firewall.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusEnum {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "deleted")]
    Deleted,
}

impl Default for StatusEnum {
    fn default() -> StatusEnum {
        Self::Enabled
    }
}

