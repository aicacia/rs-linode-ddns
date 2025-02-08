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

/// GetAccountLogins200ResponseDataInner : An object representing a previous successful login for a User.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAccountLogins200ResponseDataInner {
    /// __Read-only__ When the login was initiated.
    #[serde(rename = "datetime", skip_serializing_if = "Option::is_none")]
    pub datetime: Option<String>,
    /// __Read-only__ The unique ID of this login object.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// __Read-only__ The remote IP address that requested the login.
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// __Read-only__ True if the User that attempted the login was a restricted User, false otherwise.
    #[serde(rename = "restricted", skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// __Read-only__ Whether the login attempt succeeded or failed.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusEnum>,
    /// __Read-only__ The username of the User that attempted the login.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl GetAccountLogins200ResponseDataInner {
    /// An object representing a previous successful login for a User.
    pub fn new() -> GetAccountLogins200ResponseDataInner {
        GetAccountLogins200ResponseDataInner {
            datetime: None,
            id: None,
            ip: None,
            restricted: None,
            status: None,
            username: None,
        }
    }
}
/// __Read-only__ Whether the login attempt succeeded or failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusEnum {
    #[serde(rename = "successful")]
    Successful,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for StatusEnum {
    fn default() -> StatusEnum {
        Self::Successful
    }
}

