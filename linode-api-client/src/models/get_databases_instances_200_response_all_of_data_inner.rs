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

/// GetDatabasesInstances200ResponseAllOfDataInner : A general Managed Database instance object containing properties that are identical for all database types.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetDatabasesInstances200ResponseAllOfDataInner {
    /// Controls access to the Managed Database.  - Individually included IP addresses or CIDR ranges can access the Managed Database while all other sources are blocked.  - A standalone value of `0.0.0.0/0` allows all IP addresses access to the Managed Database.  - An empty array (`[]`) blocks all public and private connections to the Managed Database.
    #[serde(rename = "allow_list", skip_serializing_if = "Option::is_none")]
    pub allow_list: Option<Vec<String>>,
    /// The number of Linode instance nodes deployed to the Managed Database.   - Choose `3` nodes to create a high availability cluster that consists of one primary node and two replica nodes.  - A `2` node cluster is only available with a dedicated plan. It consists of one primary node and one replica node.
    #[serde(rename = "cluster_size", skip_serializing_if = "Option::is_none")]
    pub cluster_size: Option<ClusterSizeEnum>,
    /// __Read-only__ When this Managed Database was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// __Read-only__ Whether the Managed Databases is encrypted. Currently required to be `true`.
    #[serde(rename = "encrypted", skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// __Filterable__, __Read-only__ The Managed Database engine type.
    #[serde(rename = "engine", skip_serializing_if = "Option::is_none")]
    pub engine: Option<EngineEnum>,
    #[serde(rename = "fork", skip_serializing_if = "Option::is_none")]
    pub fork: Option<models::GetDatabasesInstances200ResponseAllOfDataInnerFork>,
    #[serde(rename = "hosts", skip_serializing_if = "Option::is_none")]
    pub hosts: Option<models::GetDatabasesInstances200ResponseAllOfDataInnerHosts>,
    /// __Read-only__ A unique ID that can be used to identify and reference the Managed Database.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// __Read-only__ Append this to `https://api.linode.com` to run commands for the Managed Database.
    #[serde(rename = "instance_uri", skip_serializing_if = "Option::is_none")]
    pub instance_uri: Option<String>,
    /// __Filterable__ A unique, user-defined string referring to the Managed Database. This string needs to be unique per Managed Database engine type.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// __Read-only__ A mapping between IP addresses and strings designating them as `primary` or `failover`.
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<serde_json::Value>,
    /// __Read-only__ The oldest time to which a database can be restored.
    #[serde(rename = "oldest_restore_time", skip_serializing_if = "Option::is_none")]
    pub oldest_restore_time: Option<String>,
    /// __Filterable__, __Read-only__ The back-end platform for relational databases used by the service.
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformEnum>,
    /// __Read-only__ The access port for this Managed Database.
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// __Filterable__ The [Region](https://techdocs.akamai.com/linode-api/reference/get-regions) ID for the Managed Database.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// __Filterable__, __Read-only__ The operating status of the Managed Database.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusEnum>,
    /// __Read-only__ The total disk size of the database, in GB.
    #[serde(rename = "total_disk_size_gb", skip_serializing_if = "Option::is_none")]
    pub total_disk_size_gb: Option<i32>,
    /// __Filterable__ The Linode Instance type used by the Managed Database for its nodes.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// __Read-only__ When this Managed Database was last updated.
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(rename = "updates", skip_serializing_if = "Option::is_none")]
    pub updates: Option<models::GetDatabasesInstances200ResponseAllOfDataInnerUpdates>,
    /// __Read-only__ The amount of space currently in use in the database, in GB.
    #[serde(rename = "used_disk_size_gb", skip_serializing_if = "Option::is_none")]
    pub used_disk_size_gb: Option<i32>,
    /// __Filterable__ The Managed Database engine version.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl GetDatabasesInstances200ResponseAllOfDataInner {
    /// A general Managed Database instance object containing properties that are identical for all database types.
    pub fn new() -> GetDatabasesInstances200ResponseAllOfDataInner {
        GetDatabasesInstances200ResponseAllOfDataInner {
            allow_list: None,
            cluster_size: None,
            created: None,
            encrypted: None,
            engine: None,
            fork: None,
            hosts: None,
            id: None,
            instance_uri: None,
            label: None,
            members: None,
            oldest_restore_time: None,
            platform: None,
            port: None,
            region: None,
            status: None,
            total_disk_size_gb: None,
            r#type: None,
            updated: None,
            updates: None,
            used_disk_size_gb: None,
            version: None,
        }
    }
}
/// The number of Linode instance nodes deployed to the Managed Database.   - Choose `3` nodes to create a high availability cluster that consists of one primary node and two replica nodes.  - A `2` node cluster is only available with a dedicated plan. It consists of one primary node and one replica node.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ClusterSizeEnum {
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
    #[serde(rename = "3")]
    Variant3,
}

impl Default for ClusterSizeEnum {
    fn default() -> ClusterSizeEnum {
        Self::Variant1
    }
}
/// __Filterable__, __Read-only__ The Managed Database engine type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EngineEnum {
    #[serde(rename = "mysql")]
    Mysql,
    #[serde(rename = "postgresql")]
    Postgresql,
}

impl Default for EngineEnum {
    fn default() -> EngineEnum {
        Self::Mysql
    }
}
/// __Filterable__, __Read-only__ The back-end platform for relational databases used by the service.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlatformEnum {
    #[serde(rename = "rdbms-legacy")]
    RdbmsLegacy,
    #[serde(rename = "rdbms-default")]
    RdbmsDefault,
}

impl Default for PlatformEnum {
    fn default() -> PlatformEnum {
        Self::RdbmsLegacy
    }
}
/// __Filterable__, __Read-only__ The operating status of the Managed Database.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusEnum {
    #[serde(rename = "provisioning")]
    Provisioning,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "suspending")]
    Suspending,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "resuming")]
    Resuming,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "updating")]
    Updating,
    #[serde(rename = "resizing")]
    Resizing,
}

impl Default for StatusEnum {
    fn default() -> StatusEnum {
        Self::Provisioning
    }
}

