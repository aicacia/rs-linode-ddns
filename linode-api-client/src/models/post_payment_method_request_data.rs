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

/// PostPaymentMethodRequestData : An object representing the credit card information you have on file with Linode to make Payments against your Account.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostPaymentMethodRequestData {
    /// Your credit card number. No spaces or hyphens (`-`) allowed.
    #[serde(rename = "card_number")]
    pub card_number: String,
    /// CVV (Card Verification Value) of the credit card, typically found on the back of the card.
    #[serde(rename = "cvv")]
    pub cvv: String,
    /// A value from 1-12 representing the expiration month of your credit card.    - 1 = January   - 2 = February   - 3 = March   - Etc.
    #[serde(rename = "expiry_month")]
    pub expiry_month: i32,
    /// A four-digit integer representing the expiration year of your credit card.  The combination of `expiry_month` and `expiry_year` must result in a month/year combination of the current month or in the future. An expiration date set in the past is invalid.
    #[serde(rename = "expiry_year")]
    pub expiry_year: i32,
}

impl PostPaymentMethodRequestData {
    /// An object representing the credit card information you have on file with Linode to make Payments against your Account.
    pub fn new(card_number: String, cvv: String, expiry_month: i32, expiry_year: i32) -> PostPaymentMethodRequestData {
        PostPaymentMethodRequestData {
            card_number,
            cvv,
            expiry_month,
            expiry_year,
        }
    }
}

