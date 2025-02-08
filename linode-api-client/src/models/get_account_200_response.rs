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

/// GetAccount200Response : Account object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAccount200Response {
    #[serde(rename = "active_promotions", skip_serializing_if = "Option::is_none")]
    pub active_promotions: Option<Vec<models::GetAccount200ResponseActivePromotionsInner>>,
    /// __Read-only__ The date and time the account was activated.
    #[serde(rename = "active_since", skip_serializing_if = "Option::is_none")]
    pub active_since: Option<String>,
    /// The first line of this account's billing address.
    #[serde(rename = "address_1", skip_serializing_if = "Option::is_none")]
    pub address_1: Option<String>,
    /// The second line of this account's billing address.
    #[serde(rename = "address_2", skip_serializing_if = "Option::is_none")]
    pub address_2: Option<String>,
    /// __Read-only__ This account's balance, in US dollars.
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<f64>,
    /// __Read-only__ This account's current estimated invoice in US dollars. This is not your final invoice balance. Transfer charges are not included in the estimate.
    #[serde(rename = "balance_uninvoiced", skip_serializing_if = "Option::is_none")]
    pub balance_uninvoiced: Option<f64>,
    /// __Read-only__ The source of service charges for this account. Accounts that are associated with Akamai-specific customers return a value of `akamai`. All other accounts return a value of `linode`.
    #[serde(rename = "billing_source", skip_serializing_if = "Option::is_none")]
    pub billing_source: Option<BillingSourceEnum>,
    /// __Read-only__ The Akamai Cloud Computing services your account supports.
    #[serde(rename = "capabilities", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<String>>,
    /// The city for this account's `address`.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The company name assigned to this account. This value can't include the characters, `<` `>` `(` `)` `\"` `=`.
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// The two-letter ISO 3166 country code for this account's `address`.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "credit_card", skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<models::GetAccount200ResponseCreditCard>,
    /// The email address of the person assigned to this account.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// __Read-only__ An external unique identifier for this account.
    #[serde(rename = "euuid", skip_serializing_if = "Option::is_none")]
    pub euuid: Option<uuid::Uuid>,
    /// The first name of the person assigned to this account. This value can't include the characters, `<` `>` `(` `)` `\"` `=`.
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The last name of the person assigned to this account. This value can't include the characters, `<` `>` `(` `)` `\"` `=`.
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The phone number assigned to this account.
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The state or province for the `address` set for your account, if applicable.  - If the `address` is in the United States (US) or Canada (CA), this is the two-letter ISO 3166 code for the state or province.  - If it's a US military `address`, this is the abbreviation for that territory. This includes `AA` for Armed Forces Americas (excluding Canada), `AE` for Armed Forces Africa, Europe, Middle East, and Canada, or `AP` for Armed Forces Pacific.  - If outside the US or CA, this is the province associated with the account's `address`.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The tax identification number assigned to this account, used for tax calculations in a `country` that collects tax. Set to an empty string (`\"\"`) for countries that don't collect tax.  > 📘 > > This value is externally validated. If the validation is successful, a `tax_id_valid` [event](https://techdocs.akamai.com/linode-api/reference/get-events) is triggered. If unsuccessful, a `tax_id_invalid` event is triggered and an error response is issued for an operation that included it.
    #[serde(rename = "tax_id", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    /// The zip code for this account's `address`.  - It can only contain ASCII letters, numbers, and dashes (`-`).  - It can't contain more than nine letter or number characters.
    #[serde(rename = "zip", skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
}

impl GetAccount200Response {
    /// Account object.
    pub fn new() -> GetAccount200Response {
        GetAccount200Response {
            active_promotions: None,
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
/// __Read-only__ The source of service charges for this account. Accounts that are associated with Akamai-specific customers return a value of `akamai`. All other accounts return a value of `linode`.
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

