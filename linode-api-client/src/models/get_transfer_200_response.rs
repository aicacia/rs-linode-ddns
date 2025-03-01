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

/// GetTransfer200Response : An object representing your network utilization for the current month, in Gigabytes.  Certain Regions have separate utilization quotas and rates. For Region-specific network utilization data, see `region_transfers`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetTransfer200Response {
    /// __Read-only__ The amount of your transfer pool that is billable this billing cycle.
    #[serde(rename = "billable", skip_serializing_if = "Option::is_none")]
    pub billable: Option<i32>,
    /// __Read-only__ The amount of network usage allowed this billing cycle.
    #[serde(rename = "quota", skip_serializing_if = "Option::is_none")]
    pub quota: Option<i32>,
    #[serde(rename = "region_transfers", skip_serializing_if = "Option::is_none")]
    pub region_transfers: Option<Vec<models::GetTransfer200ResponseRegionTransfersInner>>,
    /// __Read-only__ The amount of network usage you have used this billing cycle.
    #[serde(rename = "used", skip_serializing_if = "Option::is_none")]
    pub used: Option<i32>,
}

impl GetTransfer200Response {
    /// An object representing your network utilization for the current month, in Gigabytes.  Certain Regions have separate utilization quotas and rates. For Region-specific network utilization data, see `region_transfers`.
    pub fn new() -> GetTransfer200Response {
        GetTransfer200Response {
            billable: None,
            quota: None,
            region_transfers: None,
            used: None,
        }
    }
}

