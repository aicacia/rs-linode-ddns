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

/// GetLinodeIps200ResponseIpv6 : __Read-only__ Information about this Linode's IPv6 addresses.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetLinodeIps200ResponseIpv6 {
    /// A list of IPv6 range objects assigned to this Linode.
    #[serde(rename = "global", skip_serializing_if = "Option::is_none")]
    pub global: Option<Vec<models::GetLinodeIps200ResponseIpv6GlobalInner>>,
    #[serde(rename = "link_local", skip_serializing_if = "Option::is_none")]
    pub link_local: Option<models::GetLinodeIps200ResponseIpv6LinkLocal>,
    #[serde(rename = "slaac", skip_serializing_if = "Option::is_none")]
    pub slaac: Option<models::GetLinodeIps200ResponseIpv6Slaac>,
}

impl GetLinodeIps200ResponseIpv6 {
    /// __Read-only__ Information about this Linode's IPv6 addresses.
    pub fn new() -> GetLinodeIps200ResponseIpv6 {
        GetLinodeIps200ResponseIpv6 {
            global: None,
            link_local: None,
            slaac: None,
        }
    }
}

