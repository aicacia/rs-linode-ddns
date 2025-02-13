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
pub struct PostObjectStorageObjectUrlRequest {
    /// The expected `Content-type` header of the request this signed URL will be valid for.  If provided, the `Content-type` header _must_ be sent with the request when this URL is used, and _must_ be the same as it was when the signed URL was created. Required for all methods _except_ `GET` or `DELETE`.
    #[serde(rename = "content_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<Option<String>>,
    /// How long this signed URL will be valid for, in seconds.  If omitted, the URL will be valid for 3600 seconds (1 hour).
    #[serde(rename = "expires_in", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<Option<i32>>,
    /// The HTTP method allowed to be used with the pre-signed URL.
    #[serde(rename = "method")]
    pub method: String,
    /// The name of the object that will be accessed with the pre-signed URL. This object need not exist, and no error will be returned if it doesn't. This behavior is useful for generating pre-signed URLs to upload new objects to by setting the `method` to `PUT`.
    #[serde(rename = "name")]
    pub name: String,
}

impl PostObjectStorageObjectUrlRequest {
    pub fn new(method: String, name: String) -> PostObjectStorageObjectUrlRequest {
        PostObjectStorageObjectUrlRequest {
            content_type: None,
            expires_in: None,
            method,
            name,
        }
    }
}

