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

use serde_with::serde_as;

/// PostLinodeInstanceRequestAllOfMetadata : __Write-only__ An object containing user-defined data relevant to the creation of Linodes.
#[serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostLinodeInstanceRequestAllOfMetadata {
    /// Base64-encoded [cloud-config](https://www.linode.com/docs/products/compute/compute-instances/guides/metadata-cloud-config/) data.  Cannot be modified after provisioning. To update, use either the [Clone a Linode](https://techdocs.akamai.com/linode-api/reference/post-clone-linode-instance) or [Rebuild a Linode](https://techdocs.akamai.com/linode-api/reference/post-rebuild-linode-instance) operations.  Must not be included when cloning to an existing Linode.  Unencoded data must not exceed 65535 bytes, or about 16kb encoded.
    #[serde_as(as = "Option<serde_with::base64::Base64>")]
    #[serde(rename = "user_data", skip_serializing_if = "Option::is_none")]
    pub user_data: Option<Vec<u8>>,
}

impl PostLinodeInstanceRequestAllOfMetadata {
    /// __Write-only__ An object containing user-defined data relevant to the creation of Linodes.
    pub fn new() -> PostLinodeInstanceRequestAllOfMetadata {
        PostLinodeInstanceRequestAllOfMetadata {
            user_data: None,
        }
    }
}

