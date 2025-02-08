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
pub struct PutLkeClusterAcl200Response {
    #[serde(rename = "acl", skip_serializing_if = "Option::is_none")]
    pub acl: Option<models::GetLkeClusterAcl200ResponseAllOfAcl1>,
}

impl PutLkeClusterAcl200Response {
    pub fn new() -> PutLkeClusterAcl200Response {
        PutLkeClusterAcl200Response {
            acl: None,
        }
    }
}

