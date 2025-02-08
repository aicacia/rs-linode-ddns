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
pub struct PostRescueLinodeInstanceRequest {
    #[serde(rename = "devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<models::PostRescueLinodeInstanceRequestDevices>,
}

impl PostRescueLinodeInstanceRequest {
    pub fn new() -> PostRescueLinodeInstanceRequest {
        PostRescueLinodeInstanceRequest {
            devices: None,
        }
    }
}

