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
pub struct PostObjectStorageBucketRequest {
    /// The Access Control Level of the bucket using a canned ACL string. For more fine-grained control of ACLs, use the S3 API directly.
    #[serde(rename = "acl", skip_serializing_if = "Option::is_none")]
    pub acl: Option<AclEnum>,
    /// If set to `false`, CORS is disabled for all origins in the bucket. For more fine-grained controls of CORS, use the S3 API directly.
    #[serde(rename = "cors_enabled", skip_serializing_if = "Option::is_none")]
    pub cors_enabled: Option<bool>,
    /// The type of `s3_endpoint` available to the active `user` in this `region`. See [Endpoint types](https://techdocs.akamai.com/cloud-computing/docs/object-storage#endpoint-type) for more information.
    #[serde(rename = "endpoint_type", skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<EndpointTypeEnum>,
    /// The name for this bucket.  * A bucket name can contain from 3 to 63 alphanumeric characters, dashes (`-`), or dots (`.`). * A bucket name can't end in a dash and you can't use two consecutive dashes. * A bucket name can't start or end in a dot, and you can't use two consecutive dots. As a best practice, only use dots if a certificate you're using with your bucket requires it. (For example, if you're using a custom TLS certificate.) * A bucket name needs to be unique in the `region` where you're creating the bucket. The API only reserves labels for the `region` where active buckets are created and stored. If you want to reserve this bucket's label in another `region`, create a new bucket with the same label in the new `region`.
    #[serde(rename = "label")]
    pub label: String,
    /// The `id` assigned to the data center ([region](https://techdocs.akamai.com/linode-api/reference/get-regions)) where this Object Storage bucket should be created.  > 📘 > > This supports legacy `clusterId` values that represented a specific region. For example, `us-east-1` is the legacy reference for the `us-east` region.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The active user's s3 endpoint URL, based on the `endpoint_type` and `region`.
    #[serde(rename = "s3_endpoint", skip_serializing_if = "Option::is_none")]
    pub s3_endpoint: Option<String>,
}

impl PostObjectStorageBucketRequest {
    pub fn new(label: String) -> PostObjectStorageBucketRequest {
        PostObjectStorageBucketRequest {
            acl: None,
            cors_enabled: None,
            endpoint_type: None,
            label,
            region: None,
            s3_endpoint: None,
        }
    }
}
/// The Access Control Level of the bucket using a canned ACL string. For more fine-grained control of ACLs, use the S3 API directly.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AclEnum {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public-read")]
    PublicRead,
    #[serde(rename = "authenticated-read")]
    AuthenticatedRead,
    #[serde(rename = "public-read-write")]
    PublicReadWrite,
}

impl Default for AclEnum {
    fn default() -> AclEnum {
        Self::Private
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

