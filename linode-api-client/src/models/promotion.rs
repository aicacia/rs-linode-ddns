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

/// Promotion : __Read-only__ Promotions generally offer a set amount of credit that can be used toward your Linode services, and the promotion expires after a specified date. As well, a monthly cap on the promotional offer is set.  Simply put, a promotion offers a certain amount of credit  month, until either the expiration date is passed, or until the total promotional credit is used, whichever comes first.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Promotion {
    /// The amount available to spend per month.
    #[serde(rename = "credit_monthly_cap", skip_serializing_if = "Option::is_none")]
    pub credit_monthly_cap: Option<String>,
    /// The total amount of credit left for this promotion.
    #[serde(rename = "credit_remaining", skip_serializing_if = "Option::is_none")]
    pub credit_remaining: Option<String>,
    /// A detailed description of this promotion.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When this promotion's credits expire.
    #[serde(rename = "expire_dt", skip_serializing_if = "Option::is_none")]
    pub expire_dt: Option<String>,
    /// The location of an image for this promotion.
    #[serde(rename = "image_url", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// The service to which this promotion applies.
    #[serde(rename = "service_type", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<ServiceTypeEnum>,
    /// Short details of this promotion.
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// The amount of credit left for this month for this promotion.
    #[serde(rename = "this_month_credit_remaining", skip_serializing_if = "Option::is_none")]
    pub this_month_credit_remaining: Option<String>,
}

impl Promotion {
    /// __Read-only__ Promotions generally offer a set amount of credit that can be used toward your Linode services, and the promotion expires after a specified date. As well, a monthly cap on the promotional offer is set.  Simply put, a promotion offers a certain amount of credit  month, until either the expiration date is passed, or until the total promotional credit is used, whichever comes first.
    pub fn new() -> Promotion {
        Promotion {
            credit_monthly_cap: None,
            credit_remaining: None,
            description: None,
            expire_dt: None,
            image_url: None,
            service_type: None,
            summary: None,
            this_month_credit_remaining: None,
        }
    }
}
/// The service to which this promotion applies.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServiceTypeEnum {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "backup")]
    Backup,
    #[serde(rename = "blockstorage")]
    Blockstorage,
    #[serde(rename = "db_mysql")]
    DbMysql,
    #[serde(rename = "ip_v4")]
    IpV4,
    #[serde(rename = "linode")]
    Linode,
    #[serde(rename = "linode_disk")]
    LinodeDisk,
    #[serde(rename = "linode_memory")]
    LinodeMemory,
    #[serde(rename = "longview")]
    Longview,
    #[serde(rename = "managed")]
    Managed,
    #[serde(rename = "nodebalancer")]
    Nodebalancer,
    #[serde(rename = "objectstorage")]
    Objectstorage,
    #[serde(rename = "placement_group")]
    PlacementGroup,
    #[serde(rename = "transfer_tx")]
    TransferTx,
}

impl Default for ServiceTypeEnum {
    fn default() -> ServiceTypeEnum {
        Self::All
    }
}

