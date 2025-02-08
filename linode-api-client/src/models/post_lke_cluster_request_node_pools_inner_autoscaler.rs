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

/// PostLkeClusterRequestNodePoolsInnerAutoscaler : When enabled, the number of nodes automatically scales within the defined minimum and maximum values. When making a request, `max` and `min` require each other.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostLkeClusterRequestNodePoolsInnerAutoscaler {
    /// Whether automatic scaling is enabled for this node pool. Defaults to `false`.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The maximum number of nodes to automatically scale to. Defaults to the value provided by the `count` field.
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    /// The minimum number of nodes to automatically scale to. Defaults to the node pool's `count`.
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

impl PostLkeClusterRequestNodePoolsInnerAutoscaler {
    /// When enabled, the number of nodes automatically scales within the defined minimum and maximum values. When making a request, `max` and `min` require each other.
    pub fn new() -> PostLkeClusterRequestNodePoolsInnerAutoscaler {
        PostLkeClusterRequestNodePoolsInnerAutoscaler {
            enabled: None,
            max: None,
            min: None,
        }
    }
}

