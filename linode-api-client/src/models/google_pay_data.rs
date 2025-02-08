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

/// GooglePayData : Google Pay information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GooglePayData {
    /// __Read-only__ The type of credit card.
    #[serde(rename = "card_type", skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
    /// __Read-only__ The expiration month and year of the credit card.
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    /// __Read-only__ The last four digits of the credit card number.
    #[serde(rename = "last_four", skip_serializing_if = "Option::is_none")]
    pub last_four: Option<String>,
}

impl GooglePayData {
    /// Google Pay information.
    pub fn new() -> GooglePayData {
        GooglePayData {
            card_type: None,
            expiry: None,
            last_four: None,
        }
    }
}

