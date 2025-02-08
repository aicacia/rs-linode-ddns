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
pub struct PostPaymentRequest {
    /// The ID of the Payment Method to apply to the Payment.
    #[serde(rename = "payment_method_id", skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<i32>,
    /// The amount in US Dollars of the Payment.  - Can begin with or without `$`. - Commas (`,`) are not accepted. - Must end with a decimal expression, such as `.00` or `.99`. - Minimum: `$5.00` or the Account balance, whichever is lower. - Maximum: `$2000.00` or the Account balance up to `$50000.00`, whichever is greater.
    #[serde(rename = "usd", skip_serializing_if = "Option::is_none")]
    pub usd: Option<String>,
}

impl PostPaymentRequest {
    pub fn new() -> PostPaymentRequest {
        PostPaymentRequest {
            payment_method_id: None,
            usd: None,
        }
    }
}

