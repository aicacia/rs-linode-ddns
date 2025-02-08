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

/// GetManagedContacts200ResponseDataInnerPhone : Information about how to reach this Contact by phone.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetManagedContacts200ResponseDataInnerPhone {
    /// This Contact's primary phone number.
    #[serde(rename = "primary", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub primary: Option<Option<String>>,
    /// This Contact's secondary phone number.
    #[serde(rename = "secondary", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub secondary: Option<Option<String>>,
}

impl GetManagedContacts200ResponseDataInnerPhone {
    /// Information about how to reach this Contact by phone.
    pub fn new() -> GetManagedContacts200ResponseDataInnerPhone {
        GetManagedContacts200ResponseDataInnerPhone {
            primary: None,
            secondary: None,
        }
    }
}

