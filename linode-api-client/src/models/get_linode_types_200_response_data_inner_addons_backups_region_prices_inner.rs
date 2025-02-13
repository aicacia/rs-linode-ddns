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
pub struct GetLinodeTypes200ResponseDataInnerAddonsBackupsRegionPricesInner {
    /// The hourly cost for the Backups service in this region, in U.S. dollars.
    #[serde(rename = "hourly", skip_serializing_if = "Option::is_none")]
    pub hourly: Option<f64>,
    /// The unique identifier for the region.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The monthly cost for the Backups service in this region, in U.S. dollars.
    #[serde(rename = "monthly", skip_serializing_if = "Option::is_none")]
    pub monthly: Option<f64>,
}

impl GetLinodeTypes200ResponseDataInnerAddonsBackupsRegionPricesInner {
    pub fn new() -> GetLinodeTypes200ResponseDataInnerAddonsBackupsRegionPricesInner {
        GetLinodeTypes200ResponseDataInnerAddonsBackupsRegionPricesInner {
            hourly: None,
            id: None,
            monthly: None,
        }
    }
}

