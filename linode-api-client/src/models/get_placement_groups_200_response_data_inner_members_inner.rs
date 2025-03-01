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
pub struct GetPlacementGroups200ResponseDataInnerMembersInner {
    /// The compliance status of each individual compute instance in the placement group.
    #[serde(rename = "is_compliant", skip_serializing_if = "Option::is_none")]
    pub is_compliant: Option<bool>,
    /// __Read-only__ The unique identifier for a compute instance included in the placement group.
    #[serde(rename = "linode_id", skip_serializing_if = "Option::is_none")]
    pub linode_id: Option<i32>,
}

impl GetPlacementGroups200ResponseDataInnerMembersInner {
    pub fn new() -> GetPlacementGroups200ResponseDataInnerMembersInner {
        GetPlacementGroups200ResponseDataInnerMembersInner {
            is_compliant: None,
            linode_id: None,
        }
    }
}

