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

/// PostLinodeConfigInterfacesRequest : Linode Configuration Interfaces Order request object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostLinodeConfigInterfacesRequest {
    /// An ordered array of existing Configuration Profile Interface `id`s.  - All current Interface `id`s must be present in the array. - If the Configuration Profile contains Interfaces and is active on the Linode, the Linode must first be shut down prior to running this operation. - Reordering takes effect after rebooting the Linode with this Configuration Profile.  The position in the array determines which of the Linode's network Interfaces is configured:  - First [0]:  eth0 - Second [1]: eth1 - Third [2]:  eth2
    #[serde(rename = "ids")]
    pub ids: Vec<i32>,
}

impl PostLinodeConfigInterfacesRequest {
    /// Linode Configuration Interfaces Order request object.
    pub fn new(ids: Vec<i32>) -> PostLinodeConfigInterfacesRequest {
        PostLinodeConfigInterfacesRequest {
            ids,
        }
    }
}

