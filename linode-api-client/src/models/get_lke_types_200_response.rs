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
pub struct GetLkeTypes200Response {
    /// The Kubernetes types.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::GetLkeTypes200ResponseDataInner>>,
    /// __Read-only__ The current [page](https://techdocs.akamai.com/linode-api/reference/pagination).
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// __Read-only__ The total number of [pages](https://techdocs.akamai.com/linode-api/reference/pagination).
    #[serde(rename = "pages", skip_serializing_if = "Option::is_none")]
    pub pages: Option<i32>,
    /// __Read-only__ The total number of results.
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<i32>,
}

impl GetLkeTypes200Response {
    pub fn new() -> GetLkeTypes200Response {
        GetLkeTypes200Response {
            data: None,
            page: None,
            pages: None,
            results: None,
        }
    }
}

