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
pub struct GetObjectStorageBucketAcl200Response {
    /// The Access Control Level of the bucket, as a canned ACL string. For more fine-grained control of ACLs, use the S3 API directly.
    #[serde(rename = "acl", skip_serializing_if = "Option::is_none")]
    pub acl: Option<AclEnum>,
    /// The full XML of the object's ACL policy.
    #[serde(rename = "acl_xml", skip_serializing_if = "Option::is_none")]
    pub acl_xml: Option<String>,
}

impl GetObjectStorageBucketAcl200Response {
    pub fn new() -> GetObjectStorageBucketAcl200Response {
        GetObjectStorageBucketAcl200Response {
            acl: None,
            acl_xml: None,
        }
    }
}
/// The Access Control Level of the bucket, as a canned ACL string. For more fine-grained control of ACLs, use the S3 API directly.
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
    #[serde(rename = "custom")]
    Custom,
}

impl Default for AclEnum {
    fn default() -> AclEnum {
        Self::Private
    }
}

