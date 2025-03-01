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

/// GetNodeBalancerConfigNodes200ResponseDataInner : A NodeBalancer node represents a single backend serving requests for a single port of a NodeBalancer.  Nodes are specific to NodeBalancer configs, and serve traffic over their private IP.  If the same Linode is serving traffic for more than one port on the same NodeBalancer, one NodeBalancer node is required for each config (port) it should serve requests on.  For example, if you have four backends, and each should respond to both HTTP and HTTPS requests, you will need two NodeBalancer configs (port 80 and port 443) and four backends each, one for each of the Linodes serving requests for that port.
/// A NodeBalancer node represents a single backend serving requests for a single port of a NodeBalancer.  Nodes are specific to NodeBalancer configs, and serve traffic over their private IP.  If the same Linode is serving traffic for more than one port on the same NodeBalancer, one NodeBalancer node is required for each config (port) it should serve requests on.  For example, if you have four backends, and each should respond to both HTTP and HTTPS requests, you will need two NodeBalancer configs (port 80 and port 443) and four backends each, one for each of the Linodes serving requests for that port.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNodeBalancerConfigNodes200ResponseDataInner {
    TcpHttpOrHttpsConfig(models::TcpHttpOrHttpsConfig),
}

impl Default for GetNodeBalancerConfigNodes200ResponseDataInner {
    fn default() -> Self {
        Self::TcpHttpOrHttpsConfig(Default::default())
    }
}
/// The mode this NodeBalancer should use when sending traffic to this backend.  - If set to `accept` this backend is accepting traffic. - If set to `reject` this backend will not receive traffic. - If set to `drain` this backend will not receive _new_ traffic, but connections already pinned to it will continue to be routed to it. - If set to `backup`, this backend will only receive traffic if all `accept` nodes are down.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ModeEnum {
    #[serde(rename = "accept")]
    Accept,
    #[serde(rename = "reject")]
    Reject,
    #[serde(rename = "drain")]
    Drain,
    #[serde(rename = "backup")]
    Backup,
}

impl Default for ModeEnum {
    fn default() -> ModeEnum {
        Self::Accept
    }
}
/// __Read-only__ The current status of this node, based on the configured checks of its NodeBalancer Config.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusEnum {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "UP")]
    Up,
    #[serde(rename = "DOWN")]
    Down,
}

impl Default for StatusEnum {
    fn default() -> StatusEnum {
        Self::Unknown
    }
}

