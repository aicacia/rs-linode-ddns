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

/// PostAllocateIp200ResponseVpcNat11 : IPv4 address configured as a 1:1 NAT for this Interface. If no address is configured as a 1:1 NAT, `null` is returned.  __Note__. Only allowed for `vpc` type Interfaces.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostAllocateIp200ResponseVpcNat11 {
    /// The IPv4 address that is configured as a 1:1 NAT for this VPC interface.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The `id` of the VPC Subnet for this Interface.
    #[serde(rename = "subnet_id", skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<i32>,
    /// __Read-only__ The `id` of the VPC configured for this Interface.
    #[serde(rename = "vpc_id", skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<i32>,
}

impl PostAllocateIp200ResponseVpcNat11 {
    /// IPv4 address configured as a 1:1 NAT for this Interface. If no address is configured as a 1:1 NAT, `null` is returned.  __Note__. Only allowed for `vpc` type Interfaces.
    pub fn new() -> PostAllocateIp200ResponseVpcNat11 {
        PostAllocateIp200ResponseVpcNat11 {
            address: None,
            subnet_id: None,
            vpc_id: None,
        }
    }
}

