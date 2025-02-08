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

/// PutVpcSubnetRequest : A VPC Subnet Update request object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PutVpcSubnetRequest {
    /// __Filterable__ The VPC Subnet's label, for display purposes only.  - Must be unique among the VPC's Subnets. - Can only contain ASCII letters, numbers, and hyphens (`-`). You can't use two consecutive hyphens (`--`).
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl PutVpcSubnetRequest {
    /// A VPC Subnet Update request object.
    pub fn new() -> PutVpcSubnetRequest {
        PutVpcSubnetRequest {
            label: None,
        }
    }
}

