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
pub struct GetBackups200ResponseAutomaticInner {
    /// __Read-only__ Whether this Backup is available for restoration.  Backups undergoing maintenance are not available for restoration.
    #[serde(rename = "available", skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    /// __Read-only__ A list of the labels of the Configuration profiles that are part of the Backup.
    #[serde(rename = "configs", skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<String>>,
    /// __Read-only__ The date the Backup was taken.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// __Read-only__ A list of the disks that are part of the Backup.
    #[serde(rename = "disks", skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<models::GetBackups200ResponseAutomaticInnerAllOfDisksInner>>,
    /// __Read-only__ The date the Backup completed.
    #[serde(rename = "finished", skip_serializing_if = "Option::is_none")]
    pub finished: Option<String>,
    /// __Read-only__ The unique ID of this Backup.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// A label for Backups that are of type `snapshot`.
    #[serde(rename = "label", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub label: Option<Option<String>>,
    /// __Read-only__ The current state of a specific Backup.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusEnum>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// __Read-only__ The date the Backup was most recently updated.
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

impl GetBackups200ResponseAutomaticInner {
    pub fn new() -> GetBackups200ResponseAutomaticInner {
        GetBackups200ResponseAutomaticInner {
            available: None,
            configs: None,
            created: None,
            disks: None,
            finished: None,
            id: None,
            label: None,
            status: None,
            r#type: None,
            updated: None,
        }
    }
}
/// __Read-only__ The current state of a specific Backup.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusEnum {
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "needsPostProcessing")]
    NeedsPostProcessing,
    #[serde(rename = "successful")]
    Successful,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "userAborted")]
    UserAborted,
}

impl Default for StatusEnum {
    fn default() -> StatusEnum {
        Self::Paused
    }
}

