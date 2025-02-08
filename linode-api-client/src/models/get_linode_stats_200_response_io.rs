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

/// GetLinodeStats200ResponseIo : Input/Output statistics.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetLinodeStats200ResponseIo {
    /// Block/s written.
    #[serde(rename = "io", skip_serializing_if = "Option::is_none")]
    pub io: Option<Vec<Vec<f64>>>,
    /// Block/s written.
    #[serde(rename = "swap", skip_serializing_if = "Option::is_none")]
    pub swap: Option<Vec<Vec<f64>>>,
}

impl GetLinodeStats200ResponseIo {
    /// Input/Output statistics.
    pub fn new() -> GetLinodeStats200ResponseIo {
        GetLinodeStats200ResponseIo {
            io: None,
            swap: None,
        }
    }
}

