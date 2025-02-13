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

/// GetChildAccounts200ResponseDataInner : Child account object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetChildAccounts200ResponseDataInner {
    /// __Read-only__ The activation date and time for the child account.
    #[serde(rename = "active_since", skip_serializing_if = "Option::is_none")]
    pub active_since: Option<String>,
    /// __Filterable__ First line of this child account's billing address.
    #[serde(rename = "address_1", skip_serializing_if = "Option::is_none")]
    pub address_1: Option<String>,
    /// __Filterable__ Second line of this child account's billing address, if applicable.
    #[serde(rename = "address_2", skip_serializing_if = "Option::is_none")]
    pub address_2: Option<String>,
    /// __Read-only__ This child account's balance, in US dollars.
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<f64>,
    /// __Read-only__ This child account's current estimated invoice in US dollars. This is not your final invoice balance. Transfer charges are not included in the estimate.
    #[serde(rename = "balance_uninvoiced", skip_serializing_if = "Option::is_none")]
    pub balance_uninvoiced: Option<f64>,
    /// __Read-only__ The source of service charges for this account, as determined by its relationship with Akamai. The API returns a value of `external` to describe a child account in a parent-child account environment.
    #[serde(rename = "billing_source", skip_serializing_if = "Option::is_none")]
    pub billing_source: Option<BillingSourceEnum>,
    /// __Read-only__ A list of the capabilities the child account supports.
    #[serde(rename = "capabilities", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<String>>,
    /// __Filterable__ The city for this child account's billing address.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// __Filterable__ The company name for the owner of this child account. It can't include any of these characters: `<` `>` `(` `)` `\"` `=`. You can't change this value yourself. We use it to create the proxy users that a parent account uses to access a child account. Talk to your account team if you need to change this value.
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// __Filterable__ The two-letter ISO 3166 country code for this child account's billing address.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "credit_card", skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<models::GetChildAccounts200ResponseDataInnerCreditCard>,
    /// __Filterable__ The email address of the owner of this child account.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// __Read-only__ An external, unique identifier that Akamai assigned to the child account.
    #[serde(rename = "euuid", skip_serializing_if = "Option::is_none")]
    pub euuid: Option<uuid::Uuid>,
    /// __Filterable__ The first name of the owner of this child account. It can't include any of these characters: `<` `>` `(` `)` `\"` `=`.
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// __Filterable__ The last name of the owner of this child account. It can't include any of these characters: `<` `>` `(` `)` `\"` `=`.
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// __Filterable__ The phone number for the owner of this child account.
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// __Filterable__ The state or province for the billing address (`address_1` and `address_2, if applicable`). If in the United States (US) or Canada (CA), this is the two-letter ISO 3166 State or Province code.  __Note__. If this is a US military address, use state abbreviations (AA, AE, AP).
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The tax identification number for this child account. Use this for tax calculations in some countries. If you live in a country that doesn't collect taxes, ensure this is an empty string (`\"\"`).
    #[serde(rename = "tax_id", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    /// __Filterable__ The zip code of this Account's billing address. The following restrictions apply:  - Can only contain ASCII letters, numbers, and hyphens (`-`). - Can't contain more than 9 letter or number characters.
    #[serde(rename = "zip", skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
}

impl GetChildAccounts200ResponseDataInner {
    /// Child account object.
    pub fn new() -> GetChildAccounts200ResponseDataInner {
        GetChildAccounts200ResponseDataInner {
            active_since: None,
            address_1: None,
            address_2: None,
            balance: None,
            balance_uninvoiced: None,
            billing_source: None,
            capabilities: None,
            city: None,
            company: None,
            country: None,
            credit_card: None,
            email: None,
            euuid: None,
            first_name: None,
            last_name: None,
            phone: None,
            state: None,
            tax_id: None,
            zip: None,
        }
    }
}
/// __Read-only__ The source of service charges for this account, as determined by its relationship with Akamai. The API returns a value of `external` to describe a child account in a parent-child account environment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BillingSourceEnum {
    #[serde(rename = "external")]
    External,
}

impl Default for BillingSourceEnum {
    fn default() -> BillingSourceEnum {
        Self::External
    }
}

