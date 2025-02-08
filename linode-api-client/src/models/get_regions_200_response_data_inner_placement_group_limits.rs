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

/// GetRegions200ResponseDataInnerPlacementGroupLimits : __Read-only__ The limits for [placement groups](https://www.linode.com/docs/products/compute/compute-instances/guides/placement-groups/) in this region.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetRegions200ResponseDataInnerPlacementGroupLimits {
    /// __Read-only__ The maximum number of compute instances you can include in a single placement group in this region.
    #[serde(rename = "maximum_linodes_per_pg", skip_serializing_if = "Option::is_none")]
    pub maximum_linodes_per_pg: Option<i32>,
    /// __Read-only__ The maximum number of placement groups you can have in this region. Displayed as `null` if you don't have a limit.
    #[serde(rename = "maximum_pgs_per_customer", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub maximum_pgs_per_customer: Option<Option<i32>>,
}

impl GetRegions200ResponseDataInnerPlacementGroupLimits {
    /// __Read-only__ The limits for [placement groups](https://www.linode.com/docs/products/compute/compute-instances/guides/placement-groups/) in this region.
    pub fn new() -> GetRegions200ResponseDataInnerPlacementGroupLimits {
        GetRegions200ResponseDataInnerPlacementGroupLimits {
            maximum_linodes_per_pg: None,
            maximum_pgs_per_customer: None,
        }
    }
}

