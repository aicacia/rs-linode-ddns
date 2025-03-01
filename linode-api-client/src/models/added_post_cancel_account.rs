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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddedPostCancelAccount {
    /// Any reason for cancelling the account, and any other comments you might have about your Linode service.
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

impl AddedPostCancelAccount {
    pub fn new() -> AddedPostCancelAccount {
        AddedPostCancelAccount {
            comments: None,
        }
    }
}

