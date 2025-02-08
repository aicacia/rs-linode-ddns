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

/// PostLkeClusterRequestControlPlaneAcl : Defines settings related to the IP-based ACL of the LKE cluster. The object requires the `enabled` and `addresses` keys. It also supports the optional key `revision-id`. The default policy is set to `ALLOW`, so that access controls are disabled. An empty object value (`{}`) sets default elements.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostLkeClusterRequestControlPlaneAcl {
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<models::PostLkeClusterRequestControlPlaneAclAddresses>,
    /// Defines a default policy. A value of `true` results in a default policy of `DENY`. A value of `false` results in a default policy of `ALLOW`, such as for disabled access controls. It defaults to `true`. Creating a cluster with ACL, or upgrading a cluster to use ACL for LKE, is an irreversible change. Once upgraded, you can only toggle access controls with this field.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Enables clients to track events related to ACL update requests and enforcements. Optional field. If omitted, defaults to a randomly generated string.
    #[serde(rename = "revision-id", skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

impl PostLkeClusterRequestControlPlaneAcl {
    /// Defines settings related to the IP-based ACL of the LKE cluster. The object requires the `enabled` and `addresses` keys. It also supports the optional key `revision-id`. The default policy is set to `ALLOW`, so that access controls are disabled. An empty object value (`{}`) sets default elements.
    pub fn new() -> PostLkeClusterRequestControlPlaneAcl {
        PostLkeClusterRequestControlPlaneAcl {
            addresses: None,
            enabled: None,
            revision_id: None,
        }
    }
}

