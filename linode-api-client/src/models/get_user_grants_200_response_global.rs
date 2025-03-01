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

/// GetUserGrants200ResponseGlobal : A structure containing the Account-level grants a User has.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUserGrants200ResponseGlobal {
    /// The level of access this User has to Account-level actions, like billing information. A restricted User will never be able to manage users.  __Parent and child accounts__  In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, this grant can be added to a child account user, to give the user `read-write` access. This gives the child user unrestricted access to expected management operations, such as creating other child users. However, child users don't have write access to billing operations. The API issues a specific error message if a write operation is attempted by a child user.
    #[serde(rename = "account_access", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_access: Option<Option<AccountAccessEnum>>,
    /// If `true`, this User may add Managed Databases.
    #[serde(rename = "add_databases", skip_serializing_if = "Option::is_none")]
    pub add_databases: Option<bool>,
    /// If `true`, this User may add Domains.
    #[serde(rename = "add_domains", skip_serializing_if = "Option::is_none")]
    pub add_domains: Option<bool>,
    /// If `true`, this User may add Firewalls.
    #[serde(rename = "add_firewalls", skip_serializing_if = "Option::is_none")]
    pub add_firewalls: Option<bool>,
    /// If `true`, this User may add Images.
    #[serde(rename = "add_images", skip_serializing_if = "Option::is_none")]
    pub add_images: Option<bool>,
    /// If `true`, this User may create Linodes.
    #[serde(rename = "add_linodes", skip_serializing_if = "Option::is_none")]
    pub add_linodes: Option<bool>,
    /// If `true`, this User may create Longview clients and view the current plan.
    #[serde(rename = "add_longview", skip_serializing_if = "Option::is_none")]
    pub add_longview: Option<bool>,
    /// If `true`, this User may add NodeBalancers.
    #[serde(rename = "add_nodebalancers", skip_serializing_if = "Option::is_none")]
    pub add_nodebalancers: Option<bool>,
    /// If `true`, this User may add Placement Groups.
    #[serde(rename = "add_placement_groups", skip_serializing_if = "Option::is_none")]
    pub add_placement_groups: Option<bool>,
    /// If `true`, this User may add StackScripts.
    #[serde(rename = "add_stackscripts", skip_serializing_if = "Option::is_none")]
    pub add_stackscripts: Option<bool>,
    /// If `true`, this User may add Volumes.
    #[serde(rename = "add_volumes", skip_serializing_if = "Option::is_none")]
    pub add_volumes: Option<bool>,
    /// If `true`, this User may add VPCs.
    #[serde(rename = "add_vpcs", skip_serializing_if = "Option::is_none")]
    pub add_vpcs: Option<bool>,
    /// If `true`, this User may cancel the entire Account.
    #[serde(rename = "cancel_account", skip_serializing_if = "Option::is_none")]
    pub cancel_account: Option<bool>,
    /// In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, this gives a parent account access to endpoints that can be used to manage child accounts. Unrestricted parent account users have access to this grant, while restricted parent users don't. An unrestricted parent user can set this to `true` to add this grant to a restricted parent user. Displayed as `null` for all non-parent accounts.
    #[serde(rename = "child_account_access", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub child_account_access: Option<Option<bool>>,
    /// If `true`, this User may manage the Account's Longview subscription.
    #[serde(rename = "longview_subscription", skip_serializing_if = "Option::is_none")]
    pub longview_subscription: Option<bool>,
}

impl GetUserGrants200ResponseGlobal {
    /// A structure containing the Account-level grants a User has.
    pub fn new() -> GetUserGrants200ResponseGlobal {
        GetUserGrants200ResponseGlobal {
            account_access: None,
            add_databases: None,
            add_domains: None,
            add_firewalls: None,
            add_images: None,
            add_linodes: None,
            add_longview: None,
            add_nodebalancers: None,
            add_placement_groups: None,
            add_stackscripts: None,
            add_volumes: None,
            add_vpcs: None,
            cancel_account: None,
            child_account_access: None,
            longview_subscription: None,
        }
    }
}
/// The level of access this User has to Account-level actions, like billing information. A restricted User will never be able to manage users.  __Parent and child accounts__  In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, this grant can be added to a child account user, to give the user `read-write` access. This gives the child user unrestricted access to expected management operations, such as creating other child users. However, child users don't have write access to billing operations. The API issues a specific error message if a write operation is attempted by a child user.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountAccessEnum {
    #[serde(rename = "read_only")]
    ReadOnly,
    #[serde(rename = "read_write")]
    ReadWrite,
}

impl Default for AccountAccessEnum {
    fn default() -> AccountAccessEnum {
        Self::ReadOnly
    }
}

