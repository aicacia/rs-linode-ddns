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

/// AddedPostPaymentMethod : Payment Method Request Object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddedPostPaymentMethod {
    #[serde(rename = "data")]
    pub data: models::PostPaymentMethodRequestData,
    /// Whether this Payment Method is the default method for automatically processing service charges.
    #[serde(rename = "is_default")]
    pub is_default: bool,
    /// The type of Payment Method.  Alternative Payment Methods including Google Pay and PayPal can be added using the Cloud Manager. See the [Manage Payment Methods](https://www.linode.com/docs/products/platform/billing/guides/payment-methods/) guide for details and instructions.
    #[serde(rename = "type")]
    pub r#type: TypeEnum,
}

impl AddedPostPaymentMethod {
    /// Payment Method Request Object.
    pub fn new(data: models::PostPaymentMethodRequestData, is_default: bool, r#type: TypeEnum) -> AddedPostPaymentMethod {
        AddedPostPaymentMethod {
            data,
            is_default,
            r#type,
        }
    }
}
/// The type of Payment Method.  Alternative Payment Methods including Google Pay and PayPal can be added using the Cloud Manager. See the [Manage Payment Methods](https://www.linode.com/docs/products/platform/billing/guides/payment-methods/) guide for details and instructions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "credit_card")]
    CreditCard,
}

impl Default for TypeEnum {
    fn default() -> TypeEnum {
        Self::CreditCard
    }
}

