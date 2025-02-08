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

/// GetLinodeIps200ResponseIpv4VpcInner : A VPC IP address that exists in Linode's system, specific to the response for the [List VPC IP addresses](https://techdocs.akamai.com/linode-api/reference/get-vpcs-ips) operation. Returned as an empty set for Linodes that are not part of a VPC.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetLinodeIps200ResponseIpv4VpcInner {
    /// __Filterable__, __Read-only__ Returns `true` if the VPC interface is in use, meaning that the Linode was powered on using the `config_id` to which the interface belongs. Otherwise returns `false`.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// __Read-only__ An IPv4 address configured for this VPC interface. These follow the [RFC 1918](https://datatracker.ietf.org/doc/html/rfc1918) private address format. Displayed as `null` if an `address_range`.
    #[serde(rename = "address", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address: Option<Option<String>>,
    /// __Read-only__ A range of IPv4 addresses configured for this VPC interface. Displayed as `null` if a single `address`.
    #[serde(rename = "address_range", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address_range: Option<Option<String>>,
    /// __Filterable__, __Read-only__ The globally general entity identifier for the Linode configuration profile where the VPC is included.
    #[serde(rename = "config_id", skip_serializing_if = "Option::is_none")]
    pub config_id: Option<i32>,
    /// __Read-only__ The default gateway for the VPC subnet that the IP or IP range belongs to.
    #[serde(rename = "gateway", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub gateway: Option<Option<String>>,
    /// __Read-only__ The globally general API entity identifier for the Linode interface.
    #[serde(rename = "interface_id", skip_serializing_if = "Option::is_none")]
    pub interface_id: Option<i32>,
    /// __Filterable__, __Read-only__ The identifier for the Linode the VPC interface currently belongs to.
    #[serde(rename = "linode_id", skip_serializing_if = "Option::is_none")]
    pub linode_id: Option<i32>,
    /// __Read-only__ The public IP address used for NAT 1:1 with the VPC. This is empty if NAT 1:1 isn't used.
    #[serde(rename = "nat_1_1", skip_serializing_if = "Option::is_none")]
    pub nat_1_1: Option<String>,
    /// __Read-only__ The number of bits set in the `subnet_mask`.
    #[serde(rename = "prefix", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<Option<i32>>,
    /// __Filterable__, __Read-only__ The region of the VPC.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The `id` of the VPC Subnet for this interface.
    #[serde(rename = "subnet_id", skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<i32>,
    /// __Read-only__ The mask that separates host bits from network bits for the `address` or `address_range`.
    #[serde(rename = "subnet_mask", skip_serializing_if = "Option::is_none")]
    pub subnet_mask: Option<String>,
    /// __Filterable__, __Read-only__ The unique globally general API entity identifier for the VPC.
    #[serde(rename = "vpc_id", skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<i32>,
}

impl GetLinodeIps200ResponseIpv4VpcInner {
    /// A VPC IP address that exists in Linode's system, specific to the response for the [List VPC IP addresses](https://techdocs.akamai.com/linode-api/reference/get-vpcs-ips) operation. Returned as an empty set for Linodes that are not part of a VPC.
    pub fn new() -> GetLinodeIps200ResponseIpv4VpcInner {
        GetLinodeIps200ResponseIpv4VpcInner {
            active: None,
            address: None,
            address_range: None,
            config_id: None,
            gateway: None,
            interface_id: None,
            linode_id: None,
            nat_1_1: None,
            prefix: None,
            region: None,
            subnet_id: None,
            subnet_mask: None,
            vpc_id: None,
        }
    }
}

