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
pub struct PostIpv6RangeRequest {
    /// The ID of the Linode to assign this range to. The SLAAC address for the provided Linode is used as the range's `route_target`.  - __Required__ if `route_target` is omitted from the request.  - Mutually exclusive with `route_target`. Submitting values for both properties in a request results in an error.
    #[serde(rename = "linode_id", skip_serializing_if = "Option::is_none")]
    pub linode_id: Option<i32>,
    /// The prefix length of the IPv6 range.
    #[serde(rename = "prefix_length")]
    pub prefix_length: PrefixLengthEnum,
    /// The IPv6 SLAAC address to assign this range to.  - __Required__ if `linode_id` is omitted from the request.  - Mutually exclusive with `linode_id`. Submitting values for both properties in a request results in an error.  - __Note__. Omit the `/128` prefix length of the SLAAC address when using this property.
    #[serde(rename = "route_target", skip_serializing_if = "Option::is_none")]
    pub route_target: Option<String>,
}

impl PostIpv6RangeRequest {
    pub fn new(prefix_length: PrefixLengthEnum) -> PostIpv6RangeRequest {
        PostIpv6RangeRequest {
            linode_id: None,
            prefix_length,
            route_target: None,
        }
    }
}
/// The prefix length of the IPv6 range.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PrefixLengthEnum {
    #[serde(rename = "56")]
    Variant56,
    #[serde(rename = "64")]
    Variant64,
}

impl Default for PrefixLengthEnum {
    fn default() -> PrefixLengthEnum {
        Self::Variant56
    }
}

