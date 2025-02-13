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

/// GetObjectStorageBucketContent200ResponseDataInner : An Object in Object Storage, or a \"prefix\" that contains one or more objects when a `delimiter` is used.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetObjectStorageBucketContent200ResponseDataInner {
    /// An MD-5 hash of the object. `null` if this object represents a prefix.
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// The date and time this object was last modified. `null` if this object represents a prefix.
    #[serde(rename = "last_modified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    /// The name of this object or prefix.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The owner of this object, as a UUID. `null` if this object represents a prefix.
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// The size of this object, in bytes. `null` if this object represents a prefix.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

impl GetObjectStorageBucketContent200ResponseDataInner {
    /// An Object in Object Storage, or a \"prefix\" that contains one or more objects when a `delimiter` is used.
    pub fn new() -> GetObjectStorageBucketContent200ResponseDataInner {
        GetObjectStorageBucketContent200ResponseDataInner {
            etag: None,
            last_modified: None,
            name: None,
            owner: None,
            size: None,
        }
    }
}

