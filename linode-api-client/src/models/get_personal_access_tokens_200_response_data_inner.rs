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

/// GetPersonalAccessTokens200ResponseDataInner : A Personal Access Token is a token generated manually to access the API without going through an OAuth login.  Personal Access Tokens can have scopes just like OAuth tokens do, and are commonly used to give access to command-line tools like the Linode CLI, or when writing your own integrations.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetPersonalAccessTokens200ResponseDataInner {
    /// __Filterable__, __Read-only__ The date and time this token was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// __Read-only__ When this token will expire.  Personal Access Tokens cannot be renewed, so after this time the token will be completely unusable and a new token will need to be generated.  Tokens may be created with `null` as their expiry and will never expire unless revoked.
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    /// __Read-only__ This token's unique ID, which can be used to revoke it.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// __Filterable__ This token's label.  This is for display purposes only, but can be used to more easily track what you're using each token for.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// __Read-only__ The scopes this token was created with. These define what parts of the Account the token can be used to access. Many command-line tools, such as the [Linode CLI](https://github.com/linode/linode-cli), require tokens with access to `*`. Tokens with more restrictive scopes are generally more secure.
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
    /// __Read-only__ The token used to access the API.  When the token is created, the full token is returned here.  Otherwise, only the first 16 characters are returned.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl GetPersonalAccessTokens200ResponseDataInner {
    /// A Personal Access Token is a token generated manually to access the API without going through an OAuth login.  Personal Access Tokens can have scopes just like OAuth tokens do, and are commonly used to give access to command-line tools like the Linode CLI, or when writing your own integrations.
    pub fn new() -> GetPersonalAccessTokens200ResponseDataInner {
        GetPersonalAccessTokens200ResponseDataInner {
            created: None,
            expiry: None,
            id: None,
            label: None,
            scopes: None,
            token: None,
        }
    }
}

