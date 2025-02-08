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

/// GetVpcSubnets200ResponseAllOfDataInner : An object describing a VPC Subnet.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetVpcSubnets200ResponseAllOfDataInner {
    /// __Filterable__, __Read-only__ The date-time of VPC Subnet creation.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// __Filterable__, __Read-only__ The unique ID of the VPC Subnet.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// IPv4 range in CIDR canonical form.  - The range must belong to a private address space as defined in [RFC1918](https://datatracker.ietf.org/doc/html/rfc1918). - Allowed prefix lengths: 1-29. - The range must not overlap with 192.168.128.0/17. - The range must not overlap with other Subnets on the same VPC.
    #[serde(rename = "ipv4", skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<String>,
    /// __Filterable__ The VPC Subnet's label, for display purposes only.  - Must be unique among the VPC's Subnets. - Can only contain ASCII letters, numbers, and hyphens (`-`). You can't use two consecutive hyphens (`--`).
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// __Read-only__ An array of Linode IDs assigned to the VPC Subnet.  A Linode is assigned to a VPC Subnet if it has a Configuration Profile with a `vpc` purpose interface with the subnet's `subnet_id`. Even if the Configuration Profile is not active, meaning the Linode does not have access to the Subnet, the Linode still appears in this array.
    #[serde(rename = "linodes", skip_serializing_if = "Option::is_none")]
    pub linodes: Option<Vec<models::GetVpcs200ResponseAllOfDataInnerSubnetsInnerLinodesInner>>,
    /// __Filterable__, __Read-only__ The date-time of the most recent VPC Subnet update.
    #[serde(rename = "updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub updated: Option<Option<String>>,
}

impl GetVpcSubnets200ResponseAllOfDataInner {
    /// An object describing a VPC Subnet.
    pub fn new() -> GetVpcSubnets200ResponseAllOfDataInner {
        GetVpcSubnets200ResponseAllOfDataInner {
            created: None,
            id: None,
            ipv4: None,
            label: None,
            linodes: None,
            updated: None,
        }
    }
}

