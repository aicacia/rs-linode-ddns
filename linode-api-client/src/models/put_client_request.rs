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

/// PutClientRequest : A third-party application registered to Linode that users may log into with their Linode account through our authentication server at [login.linode.com](https://login.linode.com).  Using an OAuth Client, a third-party developer may be given access to some, or all, of a User's account for the purposes of their application.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PutClientRequest {
    /// __Read-only__ The OAuth Client ID.  This is used to identify the client, and is a publicly known value (it is not a secret).
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// __Filterable__ The name of this application.  This will be presented to users when they are asked to grant it access to their Account.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// __Filterable__ If this is a public or private OAuth Client.  Public clients have a slightly different authentication workflow than private clients.  See the [OAuth spec](https://oauth.net/2/) for more details.
    #[serde(rename = "public", skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    /// The location a successful log in from [login.linode.com](https://login.linode.com) should be redirected to for this client.  The receiver of this redirect should be ready to accept an OAuth exchange code and finish the OAuth exchange.
    #[serde(rename = "redirect_uri", skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    /// __Read-only__ The OAuth Client secret, used in the OAuth exchange.  This is returned as `<REDACTED>` except when an OAuth Client is created or its secret is reset.  This is a secret, and should not be shared or disclosed publicly.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// __Read-only__ The status of this application.  `active` by default.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusEnum>,
    /// __Read-only__ The URL where this client's thumbnail may be viewed, or `null` if this client does not have a thumbnail set.
    #[serde(rename = "thumbnail_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<Option<String>>,
}

impl PutClientRequest {
    /// A third-party application registered to Linode that users may log into with their Linode account through our authentication server at [login.linode.com](https://login.linode.com).  Using an OAuth Client, a third-party developer may be given access to some, or all, of a User's account for the purposes of their application.
    pub fn new() -> PutClientRequest {
        PutClientRequest {
            id: None,
            label: None,
            public: None,
            redirect_uri: None,
            secret: None,
            status: None,
            thumbnail_url: None,
        }
    }
}
/// __Read-only__ The status of this application.  `active` by default.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusEnum {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "suspended")]
    Suspended,
}

impl Default for StatusEnum {
    fn default() -> StatusEnum {
        Self::Active
    }
}

