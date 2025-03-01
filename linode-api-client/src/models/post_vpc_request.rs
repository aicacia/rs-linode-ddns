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
pub struct PostVpcRequest {
    /// A list of subnets associated with the VPC.
    #[serde(rename = "subnets", skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<models::PostVpcRequestAllOfSubnetsInner>>,
    /// __Filterable__, __Read-only__ The date-time of VPC creation.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// A written description to help distinguish the VPC.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// __Filterable__, __Read-only__ The unique ID of the VPC.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// __Filterable__ The VPC's label, for display purposes only.  - Needs to be unique among the Account's VPCs. - Can only contain ASCII letters, numbers, and hyphens (`-`). You can't use two consecutive hyphens (`--`).
    #[serde(rename = "label")]
    pub label: String,
    /// __Filterable__ The Region for the VPC.
    #[serde(rename = "region")]
    pub region: String,
    /// __Filterable__, __Read-only__ The date-time of the most recent VPC update.
    #[serde(rename = "updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub updated: Option<Option<String>>,
}

impl PostVpcRequest {
    pub fn new(label: String, region: String) -> PostVpcRequest {
        PostVpcRequest {
            subnets: None,
            created: None,
            description: None,
            id: None,
            label,
            region,
            updated: None,
        }
    }
}

