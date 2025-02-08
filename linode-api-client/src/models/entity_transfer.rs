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

/// EntityTransfer : An object representing an Entity Transfer.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityTransfer {
    /// When this transfer was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "entities", skip_serializing_if = "Option::is_none")]
    pub entities: Option<models::GetEntityTransfers200ResponseAllOfDataInnerEntities>,
    /// When this transfer expires. Transfers will automatically expire 24 hours after creation.
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    /// __Filterable__ If the requesting account created this transfer.
    #[serde(rename = "is_sender", skip_serializing_if = "Option::is_none")]
    pub is_sender: Option<bool>,
    /// __Filterable__ The status of the transfer request:  `accepted`: The transfer has been accepted by another user and is currently in progress. Transfers can take up to 3 hours to complete. `canceled`: The transfer has been canceled by the sender. `completed`: The transfer has completed successfully. `failed`: The transfer has failed after initiation. `pending`: The transfer is ready to be accepted. `stale`: The transfer has exceeded its expiration date. It can no longer be accepted or canceled.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusEnum>,
    /// The token used to identify and accept or cancel this transfer.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<uuid::Uuid>,
    /// When this transfer was last updated.
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

impl EntityTransfer {
    /// An object representing an Entity Transfer.
    pub fn new() -> EntityTransfer {
        EntityTransfer {
            created: None,
            entities: None,
            expiry: None,
            is_sender: None,
            status: None,
            token: None,
            updated: None,
        }
    }
}
/// __Filterable__ The status of the transfer request:  `accepted`: The transfer has been accepted by another user and is currently in progress. Transfers can take up to 3 hours to complete. `canceled`: The transfer has been canceled by the sender. `completed`: The transfer has completed successfully. `failed`: The transfer has failed after initiation. `pending`: The transfer is ready to be accepted. `stale`: The transfer has exceeded its expiration date. It can no longer be accepted or canceled.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusEnum {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "stale")]
    Stale,
}

impl Default for StatusEnum {
    fn default() -> StatusEnum {
        Self::Accepted
    }
}

