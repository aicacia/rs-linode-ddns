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

/// PostFirewallsRequestAllOfRulesInboundInner : One of a Firewall's inbound or outbound access rules. The `ports` property can be used to allow traffic on a comma-separated list of different ports.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostFirewallsRequestAllOfRulesInboundInner {
    /// Controls whether traffic is accepted or dropped by this rule. Overrides the Firewall's `inbound_policy` if this is an inbound rule, or the `outbound_policy` if this is an outbound rule.
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionEnum>,
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<models::PostFirewallsRequestAllOfRulesInboundInnerAddresses>,
    /// Used to describe this rule. For display purposes only.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Used to identify this rule. For display purposes only.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// A string representing the port or ports affected by this rule:  - The string may be a single port, a range of ports, or a comma-separated list of single ports and port ranges. A space is permitted following each comma. - A range of ports is inclusive of the start and end values for the range. The end value of the range must be greater than the start value. - Ports must be within 1 and 65535, and may not contain any leading zeroes. For example, port `080` is not allowed. - The ports string can have up to 15 _pieces_, where a single port is treated as one piece, and a port range is treated as two pieces. For example, the string \"22-24, 80, 443\" has four pieces. - If no ports are configured, all ports are affected. - Only allowed for the TCP and UDP protocols. Ports are not allowed for the ICMP and IPENCAP protocols.
    #[serde(rename = "ports", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Option<String>>,
    /// The type of network traffic affected by this rule.
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<ProtocolEnum>,
}

impl PostFirewallsRequestAllOfRulesInboundInner {
    /// One of a Firewall's inbound or outbound access rules. The `ports` property can be used to allow traffic on a comma-separated list of different ports.
    pub fn new() -> PostFirewallsRequestAllOfRulesInboundInner {
        PostFirewallsRequestAllOfRulesInboundInner {
            action: None,
            addresses: None,
            description: None,
            label: None,
            ports: None,
            protocol: None,
        }
    }
}
/// Controls whether traffic is accepted or dropped by this rule. Overrides the Firewall's `inbound_policy` if this is an inbound rule, or the `outbound_policy` if this is an outbound rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionEnum {
    #[serde(rename = "ACCEPT")]
    Accept,
    #[serde(rename = "DROP")]
    Drop,
}

impl Default for ActionEnum {
    fn default() -> ActionEnum {
        Self::Accept
    }
}
/// The type of network traffic affected by this rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProtocolEnum {
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
    #[serde(rename = "ICMP")]
    Icmp,
    #[serde(rename = "IPENCAP")]
    Ipencap,
}

impl Default for ProtocolEnum {
    fn default() -> ProtocolEnum {
        Self::Tcp
    }
}

