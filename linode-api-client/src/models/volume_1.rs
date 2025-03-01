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

/// Volume1 : A Block Storage Volume associated with your Account.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume1 {
    /// __Read-only__ When this Volume was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// __Limited availability__, __Read-only__ Displays if encryption is enabled on this volume.
    #[serde(rename = "encryption", skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionEnum>,
    /// __Read-only__ The full filesystem path for the Volume based on the Volume's label. Path is /dev/disk/by-id/scsi-0Linode_Volume_ + Volume label.
    #[serde(rename = "filesystem_path", skip_serializing_if = "Option::is_none")]
    pub filesystem_path: Option<String>,
    /// __Read-only__ The storage type of this Volume.
    #[serde(rename = "hardware_type", skip_serializing_if = "Option::is_none")]
    pub hardware_type: Option<HardwareTypeEnum>,
    /// __Read-only__ The unique ID of this Volume.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// __Filterable__ The Volume's label is for display purposes only.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// If a Volume is attached to a specific Linode, the ID of that Linode will be displayed here.
    #[serde(rename = "linode_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub linode_id: Option<Option<i32>>,
    /// __Read-only__ If a Volume is attached to a specific Linode, the label of that Linode will be displayed here.
    #[serde(rename = "linode_label", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub linode_label: Option<Option<String>>,
    /// __Read-only__ The unique ID of this Region.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The Volume's size, in GiB.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// __Read-only__ The current status of the volume.  Can be one of:    - `creating`. The volume is being created and is not yet available for use.   - `active`. The volume is online and available for use.   - `resizing`. The volume is in the process of upgrading its current capacity.   - `key_rotating`. The volume is in the process of rotating encryption keys. Requests to resize, delete, or clone a volume fails during encryption key rotation.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusEnum>,
    /// __Filterable__ An array of Tags applied to this object.  Tags are for organizational purposes only.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// __Read-only__ When this Volume was last updated.
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

impl Volume1 {
    /// A Block Storage Volume associated with your Account.
    pub fn new() -> Volume1 {
        Volume1 {
            created: None,
            encryption: None,
            filesystem_path: None,
            hardware_type: None,
            id: None,
            label: None,
            linode_id: None,
            linode_label: None,
            region: None,
            size: None,
            status: None,
            tags: None,
            updated: None,
        }
    }
}
/// __Limited availability__, __Read-only__ Displays if encryption is enabled on this volume.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EncryptionEnum {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

impl Default for EncryptionEnum {
    fn default() -> EncryptionEnum {
        Self::Enabled
    }
}
/// __Read-only__ The storage type of this Volume.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HardwareTypeEnum {
    #[serde(rename = "hdd")]
    Hdd,
    #[serde(rename = "nvme")]
    Nvme,
}

impl Default for HardwareTypeEnum {
    fn default() -> HardwareTypeEnum {
        Self::Hdd
    }
}
/// __Read-only__ The current status of the volume.  Can be one of:    - `creating`. The volume is being created and is not yet available for use.   - `active`. The volume is online and available for use.   - `resizing`. The volume is in the process of upgrading its current capacity.   - `key_rotating`. The volume is in the process of rotating encryption keys. Requests to resize, delete, or clone a volume fails during encryption key rotation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusEnum {
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "resizing")]
    Resizing,
    #[serde(rename = "key_rotating")]
    KeyRotating,
}

impl Default for StatusEnum {
    fn default() -> StatusEnum {
        Self::Creating
    }
}

