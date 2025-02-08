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

/// Maintenance : Information about maintenance affecting an entity.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Maintenance {
    #[serde(rename = "entity", skip_serializing_if = "Option::is_none")]
    pub entity: Option<models::GetMaintenance200ResponseDataInnerEntity>,
    /// The reason maintenance is being performed.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// __Filterable__ The maintenance status.  Maintenance progresses in the following sequence: pending, started, then completed.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusEnum>,
    /// __Filterable__ The type of maintenance.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TypeEnum>,
    /// __Filterable__ When the maintenance will begin.  [Filterable](https://techdocs.akamai.com/linode-api/reference/filtering-and-sorting) with the following parameters:  - A single value in date-time string format (`%Y-%m-%dT%H:%M:%S`), which returns only matches to that value.  - A dictionary containing pairs of inequality operator string keys (`+or`, `+gt`, `+gte`, `+lt`, `+lte`, or `+neq`) and single date-time string format values (`%Y-%m-%dT%H:%M:%S`). `+or` accepts an array of values that may consist of single date-time strings or dictionaries of inequality operator pairs.
    #[serde(rename = "when", skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,
}

impl Maintenance {
    /// Information about maintenance affecting an entity.
    pub fn new() -> Maintenance {
        Maintenance {
            entity: None,
            reason: None,
            status: None,
            r#type: None,
            when: None,
        }
    }
}
/// __Filterable__ The maintenance status.  Maintenance progresses in the following sequence: pending, started, then completed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusEnum {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "started")]
    Started,
}

impl Default for StatusEnum {
    fn default() -> StatusEnum {
        Self::Completed
    }
}
/// __Filterable__ The type of maintenance.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "reboot")]
    Reboot,
    #[serde(rename = "cold_migration")]
    ColdMigration,
    #[serde(rename = "live_migration")]
    LiveMigration,
}

impl Default for TypeEnum {
    fn default() -> TypeEnum {
        Self::Reboot
    }
}

