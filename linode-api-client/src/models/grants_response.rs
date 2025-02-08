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

/// GrantsResponse : A structure representing all grants a restricted User has on the Account. Not available for unrestricted users, as they have access to everything without grants. If retrieved from the `/profile/grants` endpoint, entities to which a User has no access will be omitted.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GrantsResponse {
    /// The grants this User has for each Database that is owned by this Account.
    #[serde(rename = "database", skip_serializing_if = "Option::is_none")]
    pub database: Option<Vec<models::GetUserGrants200ResponseDatabaseInner>>,
    /// The grants this User has for each Domain that is owned by this Account.
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<Vec<models::GetUserGrants200ResponseDatabaseInner>>,
    /// The grants this User has for each Firewall that is owned by this Account.
    #[serde(rename = "firewall", skip_serializing_if = "Option::is_none")]
    pub firewall: Option<Vec<models::GetUserGrants200ResponseDatabaseInner>>,
    #[serde(rename = "global", skip_serializing_if = "Option::is_none")]
    pub global: Option<models::GetUserGrants200ResponseGlobal>,
    /// The grants this User has for each Image that is owned by this Account.
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<Vec<models::GetUserGrants200ResponseDatabaseInner>>,
    /// The grants this User has for each Linode that is owned by this Account.
    #[serde(rename = "linode", skip_serializing_if = "Option::is_none")]
    pub linode: Option<Vec<models::GetUserGrants200ResponseDatabaseInner>>,
    /// The grants this User has for each Longview Client that is owned by this Account.
    #[serde(rename = "longview", skip_serializing_if = "Option::is_none")]
    pub longview: Option<Vec<models::GetUserGrants200ResponseDatabaseInner>>,
    /// The grants this User has for each NodeBalancer that is owned by this Account.
    #[serde(rename = "nodebalancer", skip_serializing_if = "Option::is_none")]
    pub nodebalancer: Option<Vec<models::GetUserGrants200ResponseDatabaseInner>>,
    /// The grants this User has for each Placement Group that is owned by this Account.
    #[serde(rename = "placement_group", skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<Vec<models::GetUserGrants200ResponseDatabaseInner>>,
    /// The grants this User has for each StackScript that is owned by this Account.
    #[serde(rename = "stackscript", skip_serializing_if = "Option::is_none")]
    pub stackscript: Option<Vec<models::GetUserGrants200ResponseDatabaseInner>>,
    /// The grants this User has for each Block Storage Volume that is owned by this Account.
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<Vec<models::GetUserGrants200ResponseDatabaseInner>>,
    /// The grants this User has for each VPC that is owned by this Account.
    #[serde(rename = "vpc", skip_serializing_if = "Option::is_none")]
    pub vpc: Option<Vec<models::GetUserGrants200ResponseDatabaseInner>>,
}

impl GrantsResponse {
    /// A structure representing all grants a restricted User has on the Account. Not available for unrestricted users, as they have access to everything without grants. If retrieved from the `/profile/grants` endpoint, entities to which a User has no access will be omitted.
    pub fn new() -> GrantsResponse {
        GrantsResponse {
            database: None,
            domain: None,
            firewall: None,
            global: None,
            image: None,
            linode: None,
            longview: None,
            nodebalancer: None,
            placement_group: None,
            stackscript: None,
            volume: None,
            vpc: None,
        }
    }
}

