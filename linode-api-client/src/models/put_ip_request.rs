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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PutIpRequest {
    /// The reverse DNS assigned to this address. For public IPv4 addresses, this will be set to a default value provided by Linode if not explicitly set.
    #[serde(rename = "rdns", deserialize_with = "Option::deserialize")]
    pub rdns: Option<String>,
}

impl PutIpRequest {
    pub fn new(rdns: Option<String>) -> PutIpRequest {
        PutIpRequest {
            rdns,
        }
    }
}

