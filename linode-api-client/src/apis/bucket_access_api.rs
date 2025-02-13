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


/// struct for typed errors of method [`post_object_storage_bucket_access`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostObjectStorageBucketAccessError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_storage_bucket_access`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutStorageBucketAccessError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}


/// Apply basic Cross-origin Resource Sharing (CORS) and Access Control Level (ACL) settings. You can configure CORS for all origins and set canned ACL settings.  > 📘 > > For more fine-grained control of both systems, use the [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/bucketops/#put-bucket-acl).   <<LB>>  ---   - __OAuth scopes__.      ```     object_storage:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn post_object_storage_bucket_access(configuration: &configuration::Configuration, api_version: &str, region_id: &str, bucket: &str, post_object_storage_bucket_access_request: Option<models::PostObjectStorageBucketAccessRequest>) -> Result<serde_json::Value, Error<PostObjectStorageBucketAccessError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_region_id = region_id;
    let p_bucket = bucket;
    let p_post_object_storage_bucket_access_request = post_object_storage_bucket_access_request;

    let uri_str = format!("{}/{apiVersion}/object-storage/buckets/{regionId}/{bucket}/access", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), regionId=crate::apis::urlencode(p_region_id), bucket=crate::apis::urlencode(p_bucket));
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
    req_builder = req_builder.json(&p_post_object_storage_bucket_access_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PostObjectStorageBucketAccessError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update basic Cross-origin Resource Sharing (CORS) and Access Control Level (ACL) settings. You can configure CORS for all origins and set canned ACL settings.  > 📘 > > For more fine-grained control of both systems, use the [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/bucketops/#put-bucket-acl).   <<LB>>  ---   - __OAuth scopes__.      ```     object_storage:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn put_storage_bucket_access(configuration: &configuration::Configuration, api_version: &str, region_id: &str, bucket: &str, put_storage_bucket_access_request: Option<models::PutStorageBucketAccessRequest>) -> Result<serde_json::Value, Error<PutStorageBucketAccessError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_region_id = region_id;
    let p_bucket = bucket;
    let p_put_storage_bucket_access_request = put_storage_bucket_access_request;

    let uri_str = format!("{}/{apiVersion}/object-storage/buckets/{regionId}/{bucket}/access", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), regionId=crate::apis::urlencode(p_region_id), bucket=crate::apis::urlencode(p_bucket));
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
    req_builder = req_builder.json(&p_put_storage_bucket_access_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PutStorageBucketAccessError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

