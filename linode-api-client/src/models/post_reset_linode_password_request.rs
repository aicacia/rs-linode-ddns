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
pub struct PostResetLinodePasswordRequest {
    /// The root user's password on this Linode. Linode passwords must meet a password strength score requirement that is calculated internally by the API. If the strength requirement is not met, you will receive a Password does not meet strength requirement error.
    #[serde(rename = "root_pass")]
    pub root_pass: String,
}

impl PostResetLinodePasswordRequest {
    pub fn new(root_pass: String) -> PostResetLinodePasswordRequest {
        PostResetLinodePasswordRequest {
            root_pass,
        }
    }
}

