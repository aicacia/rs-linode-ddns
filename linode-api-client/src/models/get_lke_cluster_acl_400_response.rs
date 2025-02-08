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
pub struct GetLkeClusterAcl400Response {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<models::GetLkeClusterAcl400ResponseErrorsInner>>,
}

impl GetLkeClusterAcl400Response {
    pub fn new() -> GetLkeClusterAcl400Response {
        GetLkeClusterAcl400Response {
            errors: None,
        }
    }
}

