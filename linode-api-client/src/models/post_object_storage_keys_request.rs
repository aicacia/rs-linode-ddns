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
pub struct PostObjectStorageKeysRequest {
    /// Set up the key to limit access to specific buckets, each with a specific permission level. You can create a limited Object Storage key with access to no buckets. Include an empty `bucket_access` array in the request.
    #[serde(rename = "bucket_access", skip_serializing_if = "Option::is_none")]
    pub bucket_access: Option<Vec<models::PostObjectStorageKeysRequestBucketAccessInner>>,
    /// The label for the Object Storage key, for display purposes only.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// You can use a key to create new buckets in regions set in this array. But it can't be used to manage content in those buckets. See [Create an Object Storage key](https://techdocs.akamai.com/linode-api/reference/post-object-storage-keys) for more details.
    #[serde(rename = "regions", skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

impl PostObjectStorageKeysRequest {
    pub fn new() -> PostObjectStorageKeysRequest {
        PostObjectStorageKeysRequest {
            bucket_access: None,
            label: None,
            regions: None,
        }
    }
}

