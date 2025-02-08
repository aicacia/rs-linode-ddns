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


/// struct for typed errors of method [`delete_lke_cluster_kubeconfig`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLkeClusterKubeconfigError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_lke_cluster_kubeconfig`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLkeClusterKubeconfigError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}


/// Delete and regenerate the Kubeconfig file for a Cluster.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke kubeconfig-delete 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn delete_lke_cluster_kubeconfig(configuration: &configuration::Configuration, api_version: &str, cluster_id: i32) -> Result<serde_json::Value, Error<DeleteLkeClusterKubeconfigError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_cluster_id = cluster_id;

    let uri_str = format!("{}/{apiVersion}/lke/clusters/{clusterId}/kubeconfig", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), clusterId=p_cluster_id);
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
        let entity: Option<DeleteLkeClusterKubeconfigError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Get the Kubeconfig file for a Cluster. Please note that it often takes 2-5 minutes before the Kubeconfig file is ready after first [creating a new cluster](https://techdocs.akamai.com/linode-api/reference/post-lke-cluster).   <<LB>>  ---   - __CLI__.      ```     linode-cli lke kubeconfig-view 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn get_lke_cluster_kubeconfig(configuration: &configuration::Configuration, api_version: &str, cluster_id: i32) -> Result<models::GetLkeClusterKubeconfig200Response, Error<GetLkeClusterKubeconfigError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_cluster_id = cluster_id;

    let uri_str = format!("{}/{apiVersion}/lke/clusters/{clusterId}/kubeconfig", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), clusterId=p_cluster_id);
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
        let entity: Option<GetLkeClusterKubeconfigError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

