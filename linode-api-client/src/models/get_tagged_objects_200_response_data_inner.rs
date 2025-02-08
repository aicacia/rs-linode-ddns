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
pub struct GetTaggedObjects200ResponseDataInner {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<models::GetTaggedObjects200ResponseDataInnerData>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl GetTaggedObjects200ResponseDataInner {
    pub fn new() -> GetTaggedObjects200ResponseDataInner {
        GetTaggedObjects200ResponseDataInner {
            data: None,
            r#type: None,
        }
    }
}

