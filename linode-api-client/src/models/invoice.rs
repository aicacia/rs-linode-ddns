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

/// Invoice : Account Invoice object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Invoice {
    /// __Filterable__, __Read-only__ `akamai`: This Invoice was generated according to the terms of an agreement between the customer and Akamai.  `linode`: This Invoice was generated according to the default terms, prices, and discounts.
    #[serde(rename = "billing_source", skip_serializing_if = "Option::is_none")]
    pub billing_source: Option<BillingSourceEnum>,
    /// __Filterable__, __Read-only__ When this Invoice was generated.
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// __Read-only__ The Invoice's unique ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// __Filterable__, __Read-only__ The Invoice's display label.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// __Read-only__ The amount of the Invoice before taxes in US Dollars.
    #[serde(rename = "subtotal", skip_serializing_if = "Option::is_none")]
    pub subtotal: Option<f64>,
    /// __Read-only__ The amount of tax levied on the Invoice in US Dollars.
    #[serde(rename = "tax", skip_serializing_if = "Option::is_none")]
    pub tax: Option<f64>,
    /// __Read-only__ The amount of tax broken down into subtotals by source.
    #[serde(rename = "tax_summary", skip_serializing_if = "Option::is_none")]
    pub tax_summary: Option<Vec<models::GetInvoices200ResponseDataInnerTaxSummaryInner>>,
    /// __Filterable__, __Read-only__ The amount of the Invoice after taxes in US Dollars.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
}

impl Invoice {
    /// Account Invoice object.
    pub fn new() -> Invoice {
        Invoice {
            billing_source: None,
            date: None,
            id: None,
            label: None,
            subtotal: None,
            tax: None,
            tax_summary: None,
            total: None,
        }
    }
}
/// __Filterable__, __Read-only__ `akamai`: This Invoice was generated according to the terms of an agreement between the customer and Akamai.  `linode`: This Invoice was generated according to the default terms, prices, and discounts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BillingSourceEnum {
    #[serde(rename = "akamai")]
    Akamai,
    #[serde(rename = "linode")]
    Linode,
}

impl Default for BillingSourceEnum {
    fn default() -> BillingSourceEnum {
        Self::Akamai
    }
}

