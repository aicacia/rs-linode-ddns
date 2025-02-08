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

/// Grant : Represents the level of access a restricted User has to a specific resource on the Account.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Grant {
    /// The ID of the entity this grant applies to.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// __Read-only__ The current label of the entity this grant applies to, for display purposes.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The level of access this User has to this entity.  If `null`, this User has no access.
    #[serde(rename = "permissions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Option<PermissionsEnum>>,
}

impl Grant {
    /// Represents the level of access a restricted User has to a specific resource on the Account.
    pub fn new() -> Grant {
        Grant {
            id: None,
            label: None,
            permissions: None,
        }
    }
}
/// The level of access this User has to this entity.  If `null`, this User has no access.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PermissionsEnum {
    #[serde(rename = "read_only")]
    ReadOnly,
    #[serde(rename = "read_write")]
    ReadWrite,
}

impl Default for PermissionsEnum {
    fn default() -> PermissionsEnum {
        Self::ReadOnly
    }
}

