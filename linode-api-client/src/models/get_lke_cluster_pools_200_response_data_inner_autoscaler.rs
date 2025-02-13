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

/// GetLkeClusterPools200ResponseDataInnerAutoscaler : When enabled, the number of nodes autoscales within the defined minimum and maximum values.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetLkeClusterPools200ResponseDataInnerAutoscaler {
    /// Whether autoscaling is enabled for this Node Pool. Defaults to `false`.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The maximum number of nodes to autoscale to. Defaults to the Node Pool's `count`.
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    /// The minimum number of nodes to autoscale to. Defaults to the Node Pool's `count`.
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

impl GetLkeClusterPools200ResponseDataInnerAutoscaler {
    /// When enabled, the number of nodes autoscales within the defined minimum and maximum values.
    pub fn new() -> GetLkeClusterPools200ResponseDataInnerAutoscaler {
        GetLkeClusterPools200ResponseDataInnerAutoscaler {
            enabled: None,
            max: None,
            min: None,
        }
    }
}

