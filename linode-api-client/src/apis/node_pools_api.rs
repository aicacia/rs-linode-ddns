/*
 * Akamai: Linode API
 *
 * Add a Cloud Computing instance so you can build, release, and scale applications faster with virtual machines. 
 *
 * The version of the OpenAPI document: 4.193.0
 * Contact: jperez@linode.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`delete_lke_node_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLkeNodePoolError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_lke_cluster_pools`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLkeClusterPoolsError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_lke_node_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLkeNodePoolError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_lke_cluster_pool_recycle`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostLkeClusterPoolRecycleError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_lke_cluster_pools`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostLkeClusterPoolsError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_lke_node_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutLkeNodePoolError {
    UnknownValue(serde_json::Value),
}


/// Delete a specific Node Pool from a Kubernetes cluster.  __Deleting a Node Pool is a destructive action and cannot be undone.__  Deleting a Node Pool will delete all Linodes within that Pool.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke pool-delete 12345 456     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn delete_lke_node_pool(configuration: &configuration::Configuration, api_version: &str, cluster_id: i32, pool_id: i32) -> Result<serde_json::Value, Error<DeleteLkeNodePoolError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_cluster_id = cluster_id;
    let p_pool_id = pool_id;

    let uri_str = format!("{}/{apiVersion}/lke/clusters/{clusterId}/pools/{poolId}", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), clusterId=p_cluster_id, poolId=p_pool_id);
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteLkeNodePoolError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns all active Node Pools on a Kubernetes cluster.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke pools-list 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn get_lke_cluster_pools(configuration: &configuration::Configuration, api_version: &str, cluster_id: i32) -> Result<models::GetLkeClusterPools200Response, Error<GetLkeClusterPoolsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_cluster_id = cluster_id;

    let uri_str = format!("{}/{apiVersion}/lke/clusters/{clusterId}/pools", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), clusterId=p_cluster_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetLkeClusterPoolsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Get a specific Node Pool by ID.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke pool-view 12345 456     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn get_lke_node_pool(configuration: &configuration::Configuration, api_version: &str, cluster_id: i32, pool_id: i32) -> Result<models::GetLkeClusterPools200ResponseDataInner, Error<GetLkeNodePoolError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_cluster_id = cluster_id;
    let p_pool_id = pool_id;

    let uri_str = format!("{}/{apiVersion}/lke/clusters/{clusterId}/pools/{poolId}", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), clusterId=p_cluster_id, poolId=p_pool_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetLkeNodePoolError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Recycles a Node Pool for the designated Kubernetes Cluster. All Linodes within the Node Pool will be deleted and replaced with new Linodes on a rolling basis, which may take several minutes. Replacement Nodes are installed with the latest available patch for the Cluster's Kubernetes Version.  __Any local storage on deleted Linodes (such as `hostPath` and `emptyDir` volumes, or `local` PersistentVolumes) will be erased.__   <<LB>>  ---   - __CLI__.      ```     linode-cli lke pool-recycle 12345 456     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn post_lke_cluster_pool_recycle(configuration: &configuration::Configuration, api_version: &str, cluster_id: i32, pool_id: i32) -> Result<serde_json::Value, Error<PostLkeClusterPoolRecycleError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_cluster_id = cluster_id;
    let p_pool_id = pool_id;

    let uri_str = format!("{}/{apiVersion}/lke/clusters/{clusterId}/pools/{poolId}/recycle", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), clusterId=p_cluster_id, poolId=p_pool_id);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PostLkeClusterPoolRecycleError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Creates a new Node Pool for the designated Kubernetes cluster.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke pool-create 12345 \\   --type g6-standard-4 \\   --count 6 \\   --tags example-tag \\   --autoscaler.enabled true \\   --autoscaler.max 12 \\   --autoscaler.min 3 \\   --labels '{ \"example.com/my-app\":\"team1\" }' \\   --taints.effect \"NoSchedule\" \\   --taints.key \"example.com/my-app\" \\   --taints.value \"teamA\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn post_lke_cluster_pools(configuration: &configuration::Configuration, api_version: &str, cluster_id: i32, post_lke_cluster_pools_request: models::PostLkeClusterPoolsRequest) -> Result<models::GetLkeClusterPools200ResponseDataInner, Error<PostLkeClusterPoolsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_cluster_id = cluster_id;
    let p_post_lke_cluster_pools_request = post_lke_cluster_pools_request;

    let uri_str = format!("{}/{apiVersion}/lke/clusters/{clusterId}/pools", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), clusterId=p_cluster_id);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_post_lke_cluster_pools_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PostLkeClusterPoolsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Updates a node pool's count, labels and taints, and autoscaler configuration.  Linodes are created or deleted to match changes to the Node Pool's count.  Specifying labels or taints on update overwrites any previous values, and updates existing nodes with the new values without a recycle.  __Any local storage on deleted Linodes (such as `hostPath` and `emptyDir` volumes, or `local` PersistentVolumes) will be erased.__   <<LB>>  ---   - __CLI__.      ```     linode-cli lke pool-update 12345 456 \\   --count 6 \\   --autoscaler.enabled true \\   --autoscaler.max 12 \\   --autoscaler.min 3 \\   --labels '{ \"example.com/my-app\":\"team1\", \"env\":\"staging\" }' \\   --taints.effect \"NoSchedule\" \\   --taints.key \"example.com/my-app\" \\   --taints.value \"teamA\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn put_lke_node_pool(configuration: &configuration::Configuration, api_version: &str, cluster_id: i32, pool_id: i32, put_lke_node_pool_request: Option<models::PutLkeNodePoolRequest>) -> Result<models::GetLkeClusterPools200ResponseDataInner, Error<PutLkeNodePoolError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_cluster_id = cluster_id;
    let p_pool_id = pool_id;
    let p_put_lke_node_pool_request = put_lke_node_pool_request;

    let uri_str = format!("{}/{apiVersion}/lke/clusters/{clusterId}/pools/{poolId}", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), clusterId=p_cluster_id, poolId=p_pool_id);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_put_lke_node_pool_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PutLkeNodePoolError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

