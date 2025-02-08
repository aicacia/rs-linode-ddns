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

/// PropertiesEntities : A collection of the services to include in this transfer request, separated by type.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertiesEntities {
    /// An array containing the IDs of each of the Linodes included in this transfer.
    #[serde(rename = "linodes", skip_serializing_if = "Option::is_none")]
    pub linodes: Option<Vec<i32>>,
}

impl PropertiesEntities {
    /// A collection of the services to include in this transfer request, separated by type.
    pub fn new() -> PropertiesEntities {
        PropertiesEntities {
            linodes: None,
        }
    }
}

