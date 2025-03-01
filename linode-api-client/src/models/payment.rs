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

/// Payment : Payment object response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Payment {
    /// __Read-only__ When the Payment was made.
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// __Read-only__ The unique ID of the Payment.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// __Read-only__ The amount, in US dollars, of the Payment.
    #[serde(rename = "usd", skip_serializing_if = "Option::is_none")]
    pub usd: Option<i32>,
}

impl Payment {
    /// Payment object response.
    pub fn new() -> Payment {
        Payment {
            date: None,
            id: None,
            usd: None,
        }
    }
}

