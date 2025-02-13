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
pub struct GetObjectStorageKeys200ResponseDataInnerRegionsInner {
    /// The type of `s3_endpoint` available to the active `user` in this `region`. See [Endpoint types](https://techdocs.akamai.com/cloud-computing/docs/object-storage#endpoint-type) for more information.
    #[serde(rename = "endpoint_type", skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<EndpointTypeEnum>,
    /// Identifies each region where you can use the Object Storage key.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The S3-compatible hostname you can use to access the Object Storage buckets in this region.
    #[serde(rename = "s3_endpoint", skip_serializing_if = "Option::is_none")]
    pub s3_endpoint: Option<String>,
}

impl GetObjectStorageKeys200ResponseDataInnerRegionsInner {
    pub fn new() -> GetObjectStorageKeys200ResponseDataInnerRegionsInner {
        GetObjectStorageKeys200ResponseDataInnerRegionsInner {
            endpoint_type: None,
            id: None,
            s3_endpoint: None,
        }
    }
}
/// The type of `s3_endpoint` available to the active `user` in this `region`. See [Endpoint types](https://techdocs.akamai.com/cloud-computing/docs/object-storage#endpoint-type) for more information.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EndpointTypeEnum {
    #[serde(rename = "E0")]
    E0,
    #[serde(rename = "E1")]
    E1,
    #[serde(rename = "E2")]
    E2,
    #[serde(rename = "E3")]
    E3,
}

impl Default for EndpointTypeEnum {
    fn default() -> EndpointTypeEnum {
        Self::E0
    }
}

