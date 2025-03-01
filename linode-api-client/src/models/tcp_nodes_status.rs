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

/// TcpNodesStatus : __Read-only__ Describes the health of the backends for this port. This data updates periodically as checks are performed against backends.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TcpNodesStatus {
    /// __Read-only__ The number of backends considered to be `DOWN` and unhealthy.  These are not in rotation, and not serving requests.
    #[serde(rename = "down", skip_serializing_if = "Option::is_none")]
    pub down: Option<i32>,
    /// __Read-only__ The number of backends considered to be `UP` and healthy, and that are serving requests.
    #[serde(rename = "up", skip_serializing_if = "Option::is_none")]
    pub up: Option<i32>,
}

impl TcpNodesStatus {
    /// __Read-only__ Describes the health of the backends for this port. This data updates periodically as checks are performed against backends.
    pub fn new() -> TcpNodesStatus {
        TcpNodesStatus {
            down: None,
            up: None,
        }
    }
}

