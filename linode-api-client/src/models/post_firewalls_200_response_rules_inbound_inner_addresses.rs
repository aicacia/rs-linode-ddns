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

/// PostFirewalls200ResponseRulesInboundInnerAddresses : The IPv4 and/or IPv6 addresses affected by this rule. A Rule can have up to 255 total addresses or networks listed across its IPv4 and IPv6 arrays. A network and a single IP are treated as equivalent when accounting for this limit.  Must contain `ipv4`, `ipv6`, or both.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostFirewalls200ResponseRulesInboundInnerAddresses {
    /// A list of IPv4 addresses or networks. Addresses must be in IP/mask format. Must not be an empty list.  If `0.0.0.0/0` is included in this list, all IPv4 addresses are affected by this rule.
    #[serde(rename = "ipv4", skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<Vec<String>>,
    /// A list of IPv6 addresses or networks. Addresses must be in IP/mask format and must not include zone_id notation as described in [RFC 4007](https://www.rfc-editor.org/rfc/rfc4007). Must not be an empty list.  If `::/0` is included in this list, all IPv6 addresses are affected by this rule.
    #[serde(rename = "ipv6", skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<Vec<String>>,
}

impl PostFirewalls200ResponseRulesInboundInnerAddresses {
    /// The IPv4 and/or IPv6 addresses affected by this rule. A Rule can have up to 255 total addresses or networks listed across its IPv4 and IPv6 arrays. A network and a single IP are treated as equivalent when accounting for this limit.  Must contain `ipv4`, `ipv6`, or both.
    pub fn new() -> PostFirewalls200ResponseRulesInboundInnerAddresses {
        PostFirewalls200ResponseRulesInboundInnerAddresses {
            ipv4: None,
            ipv6: None,
        }
    }
}

