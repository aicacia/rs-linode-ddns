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

/// GetLinodeTypes200ResponseDataInnerAddonsBackupsPrice : The default cost to enable the Backups service for this Linode type. Prices are in U.S. dollars, broken down into hourly and monthly charges. Different prices apply in different regions. For region-specific prices, see `region_prices`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetLinodeTypes200ResponseDataInnerAddonsBackupsPrice {
    /// The hourly cost for the Backups service, in U.S. dollars.
    #[serde(rename = "hourly", skip_serializing_if = "Option::is_none")]
    pub hourly: Option<f64>,
    /// The monthly cost for the Backups service, in U.S. dollars.
    #[serde(rename = "monthly", skip_serializing_if = "Option::is_none")]
    pub monthly: Option<f64>,
}

impl GetLinodeTypes200ResponseDataInnerAddonsBackupsPrice {
    /// The default cost to enable the Backups service for this Linode type. Prices are in U.S. dollars, broken down into hourly and monthly charges. Different prices apply in different regions. For region-specific prices, see `region_prices`.
    pub fn new() -> GetLinodeTypes200ResponseDataInnerAddonsBackupsPrice {
        GetLinodeTypes200ResponseDataInnerAddonsBackupsPrice {
            hourly: None,
            monthly: None,
        }
    }
}

