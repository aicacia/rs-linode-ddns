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

/// PutLongviewPlanRequest : Longview Plan object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PutLongviewPlanRequest {
    /// The subscription ID for a particular Longview plan. A value of `null` corresponds to Longview Free. You can send a request to the [List Longview subscriptions](https://techdocs.akamai.com/linode-api/reference/get-longview-subscriptions) operation to receive the details of each plan.
    #[serde(rename = "longview_subscription", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub longview_subscription: Option<Option<LongviewSubscriptionEnum>>,
}

impl PutLongviewPlanRequest {
    /// Longview Plan object.
    pub fn new() -> PutLongviewPlanRequest {
        PutLongviewPlanRequest {
            longview_subscription: None,
        }
    }
}
/// The subscription ID for a particular Longview plan. A value of `null` corresponds to Longview Free. You can send a request to the [List Longview subscriptions](https://techdocs.akamai.com/linode-api/reference/get-longview-subscriptions) operation to receive the details of each plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LongviewSubscriptionEnum {
    #[serde(rename = "longview-3")]
    Longview3,
    #[serde(rename = "longview-10")]
    Longview10,
    #[serde(rename = "longview-40")]
    Longview40,
    #[serde(rename = "longview-100")]
    Longview100,
}

impl Default for LongviewSubscriptionEnum {
    fn default() -> LongviewSubscriptionEnum {
        Self::Longview3
    }
}

