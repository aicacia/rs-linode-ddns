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

/// StatsAvailableCpuInner : A stat data point.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatsAvailableCpuInner {
    /// __Read-only__ A stats graph data point.
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>,
    /// __Read-only__ A stats graph data point.
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: Option<i32>,
}

impl StatsAvailableCpuInner {
    /// A stat data point.
    pub fn new() -> StatsAvailableCpuInner {
        StatsAvailableCpuInner {
            x: None,
            y: None,
        }
    }
}

