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

/// GetDatabasesTypes200ResponseAllOfDataInner : Managed Database plan type object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetDatabasesTypes200ResponseAllOfDataInner {
    /// The compute class category.
    #[serde(rename = "class", skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    /// The amount of disk space set aside for Databases of this plan type. The value is represented in megabytes.
    #[serde(rename = "disk", skip_serializing_if = "Option::is_none")]
    pub disk: Option<i32>,
    #[serde(rename = "engines", skip_serializing_if = "Option::is_none")]
    pub engines: Option<models::GetDatabasesTypes200ResponseAllOfDataInnerEngines>,
    /// __Read-only__ The ID representing the Managed Database node plan type.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// __Read-only__ A human-readable string that describes each plan type. For display purposes only.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The amount of RAM allocated to Database created of this plan type. The value is represented in megabytes.
    #[serde(rename = "memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<i32>,
    /// The number of CPUs allocated to databases of this plan type.
    #[serde(rename = "vcpus", skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<i32>,
}

impl GetDatabasesTypes200ResponseAllOfDataInner {
    /// Managed Database plan type object.
    pub fn new() -> GetDatabasesTypes200ResponseAllOfDataInner {
        GetDatabasesTypes200ResponseAllOfDataInner {
            class: None,
            disk: None,
            engines: None,
            id: None,
            label: None,
            memory: None,
            vcpus: None,
        }
    }
}

