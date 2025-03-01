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

/// PutImageRequest : Image object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PutImageRequest {
    /// __Read-only__ A list of the possible capabilities of this image.  - `cloud-init`. The image supports the cloud-init multi-distribution method with our [Metadata service](https://www.linode.com/docs/products/compute/compute-instances/guides/metadata/#troubleshoot-metadata-and-cloud-init). This only applies to public images.  - `distributed-sites`. Whether the image can be used in distributed compute regions. Compared to a core compute region, distributed compute regions offer limited functionality, but they're globally distributed. Your image can be geographically closer to you, potentially letting you deploy it quicker. See [Regions and images](https://techdocs.akamai.com/cloud-computing/docs/images#regions-and-images) for complete details.
    #[serde(rename = "capabilities", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<String>>,
    /// __Read-only__ When this image was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// __Read-only__ The name of the user who created this image, or `linode` for public images.
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// __Filterable__, __Read-only__ Whether this image is deprecated. Only public images can be deprecated.
    #[serde(rename = "deprecated", skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// A detailed description of this image.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// __Read-only__ The date of the public image's planned removal from service. This is `null` for private images.
    #[serde(rename = "eol", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub eol: Option<Option<String>>,
    /// __Read-only__ Only images created automatically from a deleted compute instance (type=automatic) expire. This is `null` for private images.
    #[serde(rename = "expiry", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<Option<String>>,
    /// __Read-only__ The unique identifier for each image.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// __Filterable__, __Read-only__ Revealed as `true` if the image is a public distribution image. Private, account-specific images are listed as `false`.
    #[serde(rename = "is_public", skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    /// __Filterable__ A short description of the image.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// __Read-only__ Details on the regions where this image is stored. See [Regions and images](https://techdocs.akamai.com/cloud-computing/docs/images#regions-and-images) for full details on support for `regions`.
    #[serde(rename = "regions", skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<models::GetImages200ResponseDataInnerRegionsInner>>,
    /// __Filterable__, __Read-only__ The minimum size in MB this image needs to deploy.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// __Filterable__, __Read-only__ The current status of the image. Possible values are `available`, `creating`, and `pending_upload`.  > 📘 > > The `+order_by` and `+order` operators are not available when [filtering](https://techdocs.akamai.com/linode-api/reference/filtering-and-sorting) on this key.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusEnum>,
    /// __Filterable__ Tags used for organizational purposes. A tag can be from 3 to 100 characters long, and an image can have a maximum of 500 total tags.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// __Read-only__ The total size in bytes of all instances of this image, in all `regions`.  > 📘 > > This object is empty for existing images. It's intended for use with future functionality.
    #[serde(rename = "total_size", skip_serializing_if = "Option::is_none")]
    pub total_size: Option<i32>,
    /// __Filterable__, __Read-only__ How the image was created. Create a `manual` image at any time. An `automatic` image is created automatically from a deleted compute instance.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TypeEnum>,
    /// __Read-only__ When this image was last updated.
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// __Filterable__, __Read-only__ The upstream distribution vendor. This is `null` for private images.
    #[serde(rename = "vendor", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<Option<String>>,
}

impl PutImageRequest {
    /// Image object.
    pub fn new() -> PutImageRequest {
        PutImageRequest {
            capabilities: None,
            created: None,
            created_by: None,
            deprecated: None,
            description: None,
            eol: None,
            expiry: None,
            id: None,
            is_public: None,
            label: None,
            regions: None,
            size: None,
            status: None,
            tags: None,
            total_size: None,
            r#type: None,
            updated: None,
            vendor: None,
        }
    }
}
/// __Filterable__, __Read-only__ The current status of the image. Possible values are `available`, `creating`, and `pending_upload`.  > 📘 > > The `+order_by` and `+order` operators are not available when [filtering](https://techdocs.akamai.com/linode-api/reference/filtering-and-sorting) on this key.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusEnum {
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "pending_upload")]
    PendingUpload,
    #[serde(rename = "available")]
    Available,
}

impl Default for StatusEnum {
    fn default() -> StatusEnum {
        Self::Creating
    }
}
/// __Filterable__, __Read-only__ How the image was created. Create a `manual` image at any time. An `automatic` image is created automatically from a deleted compute instance.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "automatic")]
    Automatic,
}

impl Default for TypeEnum {
    fn default() -> TypeEnum {
        Self::Manual
    }
}

