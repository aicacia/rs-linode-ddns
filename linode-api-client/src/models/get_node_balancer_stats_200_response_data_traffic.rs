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

/// GetNodeBalancerStats200ResponseDataTraffic : Traffic statistics for this NodeBalancer.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetNodeBalancerStats200ResponseDataTraffic {
    /// An array of key/value pairs representing unix timestamp and reading for inbound traffic.
    #[serde(rename = "in", skip_serializing_if = "Option::is_none")]
    pub r#in: Option<Vec<f64>>,
    /// An array of key/value pairs representing unix timestamp and reading for outbound traffic.
    #[serde(rename = "out", skip_serializing_if = "Option::is_none")]
    pub out: Option<Vec<f64>>,
}

impl GetNodeBalancerStats200ResponseDataTraffic {
    /// Traffic statistics for this NodeBalancer.
    pub fn new() -> GetNodeBalancerStats200ResponseDataTraffic {
        GetNodeBalancerStats200ResponseDataTraffic {
            r#in: None,
            out: None,
        }
    }
}

