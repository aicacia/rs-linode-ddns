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

/// GetLinodeConfigInterface200ResponseIpv4 : IPv4 addresses configured for this interface. This only applies to interfaces with a `purpose` of `vpc`. Returned as `null` if no `vpc` interface is assigned.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetLinodeConfigInterface200ResponseIpv4 {
    #[serde(rename = "nat_1_1", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nat_1_1: Option<Option<models::GetLinodeConfigInterface200ResponseIpv4Nat11>>,
    /// The VPC subnet IPv4 address for this interface.  - This only applies to interfaces with a `purpose` of `vpc`.  - Returned as an empty string (`\"\"`) for non-`vpc` type interfaces.  When included in a request:  - The `vpc` can't be assigned to an existing Linode as an address or in a range.  - The target address can't be the first two or last two addresses in the subnet IPv4 range.  - If omitted, a valid address within the Subnet IPv4 range is automatically assigned.
    #[serde(rename = "vpc", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vpc: Option<Option<String>>,
}

impl GetLinodeConfigInterface200ResponseIpv4 {
    /// IPv4 addresses configured for this interface. This only applies to interfaces with a `purpose` of `vpc`. Returned as `null` if no `vpc` interface is assigned.
    pub fn new() -> GetLinodeConfigInterface200ResponseIpv4 {
        GetLinodeConfigInterface200ResponseIpv4 {
            nat_1_1: None,
            vpc: None,
        }
    }
}

