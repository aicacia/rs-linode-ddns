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

/// NodeBalancerTransfer : __Read-only__ Information about the amount of transfer this NodeBalancer has had so far this month.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeBalancerTransfer {
    /// __Read-only__ The total outbound transfer, in MB, used for this NodeBalancer this month.
    #[serde(rename = "in", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#in: Option<Option<f64>>,
    /// __Read-only__ The total inbound transfer, in MB, used for this NodeBalancer this month.
    #[serde(rename = "out", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub out: Option<Option<f64>>,
    /// __Read-only__ The total transfer, in MB, used by this NodeBalancer this month.
    #[serde(rename = "total", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total: Option<Option<f64>>,
}

impl NodeBalancerTransfer {
    /// __Read-only__ Information about the amount of transfer this NodeBalancer has had so far this month.
    pub fn new() -> NodeBalancerTransfer {
        NodeBalancerTransfer {
            r#in: None,
            out: None,
            total: None,
        }
    }
}

