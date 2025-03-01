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

/// GetAccountAgreements200Response : Acknowledgment status for agreements on your account. When acknowledging any agreements, set them to `true` and omit any remainders.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAccountAgreements200Response {
    /// The acknowledgement status for the [cross-border data transfer](https://www.akamai.com/legal/compliance/privacy-trust-center/cross-border-data-transfer-statement) agreement.
    #[serde(rename = "eu_model", skip_serializing_if = "Option::is_none")]
    pub eu_model: Option<bool>,
    /// The acknowledgement status for Akamai's [master service agreement](https://www.linode.com/legal-msa/).
    #[serde(rename = "master_service_agreement", skip_serializing_if = "Option::is_none")]
    pub master_service_agreement: Option<bool>,
    /// The acknowledgement status for Akamai's [privacy statement](https://www.akamai.com/legal/privacy-statement).
    #[serde(rename = "privacy_policy", skip_serializing_if = "Option::is_none")]
    pub privacy_policy: Option<bool>,
}

impl GetAccountAgreements200Response {
    /// Acknowledgment status for agreements on your account. When acknowledging any agreements, set them to `true` and omit any remainders.
    pub fn new() -> GetAccountAgreements200Response {
        GetAccountAgreements200Response {
            eu_model: None,
            master_service_agreement: None,
            privacy_policy: None,
        }
    }
}

