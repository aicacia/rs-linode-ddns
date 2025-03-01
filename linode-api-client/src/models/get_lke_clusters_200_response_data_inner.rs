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

/// GetLkeClusters200ResponseDataInner : A Kubernetes cluster.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetLkeClusters200ResponseDataInner {
    #[serde(rename = "control_plane", skip_serializing_if = "Option::is_none")]
    pub control_plane: Option<models::GetLkeClusters200ResponseDataInnerControlPlane>,
    /// __Read-only__ When this Kubernetes cluster was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// __Read-only__ This Kubernetes cluster's unique ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// __Filterable__ The desired Kubernetes version for this Kubernetes cluster in the format of &lt;major&gt;.&lt;minor&gt;, and the latest supported patch version will be deployed.
    #[serde(rename = "k8s_version", skip_serializing_if = "Option::is_none")]
    pub k8s_version: Option<String>,
    /// __Filterable__ This Kubernetes cluster's unique label for display purposes only. Labels have the following constraints:    - UTF-8 characters will be returned by the API using escape sequences of their Unicode code points. For example, the Japanese character _か_ is 3 bytes in UTF-8 (`0xE382AB`). Its Unicode code point is 2 bytes (`0x30AB`). APIv4 supports this character and the API will return it as the escape sequence using six 1 byte characters which represent 2 bytes of Unicode code point (`\"\\u30ab\"`).    - 4 byte UTF-8 characters are not supported.    - If the label is entirely composed of UTF-8 characters, the API response will return the code points using up to 193 1 byte characters.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// __Filterable__ This Kubernetes cluster's location.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// __Filterable__ An array of tags applied to the Kubernetes cluster. Tags are for organizational purposes only.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// __Beta__, __Filterable__ The desired Kubernetes tier.  > 🚧 > > This field is in beta and only works when using the beta API. Call the URL with the `apiVersion` path parameter set to `v4beta`.
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<TierEnum>,
    /// __Read-only__ When this Kubernetes cluster was updated.
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

impl GetLkeClusters200ResponseDataInner {
    /// A Kubernetes cluster.
    pub fn new() -> GetLkeClusters200ResponseDataInner {
        GetLkeClusters200ResponseDataInner {
            control_plane: None,
            created: None,
            id: None,
            k8s_version: None,
            label: None,
            region: None,
            tags: None,
            tier: None,
            updated: None,
        }
    }
}
/// __Beta__, __Filterable__ The desired Kubernetes tier.  > 🚧 > > This field is in beta and only works when using the beta API. Call the URL with the `apiVersion` path parameter set to `v4beta`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TierEnum {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "enterprise")]
    Enterprise,
}

impl Default for TierEnum {
    fn default() -> TierEnum {
        Self::Standard
    }
}

