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


/// struct for typed errors of method [`get_object_storage_bucket_acl`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetObjectStorageBucketAclError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_object_storage_bucket_acl`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutObjectStorageBucketAclError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}


/// View an Object's configured Access Control List (ACL) in this Object Storage bucket. ACLs define who can access your buckets and objects and specify the level of access granted to those users.  > 📘 > > The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/objectops/#get-object-acl) equivalent operation offers more detail.   <<LB>>  ---   - __OAuth scopes__.      ```     object_storage:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn get_object_storage_bucket_acl(configuration: &configuration::Configuration, api_version: &str, region_id: &str, bucket: &str, name: &str) -> Result<models::GetObjectStorageBucketAcl200Response, Error<GetObjectStorageBucketAclError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_region_id = region_id;
    let p_bucket = bucket;
    let p_name = name;

    let uri_str = format!("{}/{apiVersion}/object-storage/buckets/{regionId}/{bucket}/object-acl", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), regionId=crate::apis::urlencode(p_region_id), bucket=crate::apis::urlencode(p_bucket));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("name", &p_name.to_string())]);
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
        let entity: Option<GetObjectStorageBucketAclError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update an object's configured access control level (ACL) in this Object Storage bucket. ACLs define who can access your buckets and objects, and specify the level of access granted to those users.  > 📘 > > The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/objectops/#set-object-acl) equivalent operation offers more detail.   <<LB>>  ---   - __OAuth scopes__.      ```     object_storage:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn put_object_storage_bucket_acl(configuration: &configuration::Configuration, api_version: &str, region_id: &str, bucket: &str, put_object_storage_bucket_acl_request: Option<models::PutObjectStorageBucketAclRequest>) -> Result<serde_json::Value, Error<PutObjectStorageBucketAclError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_region_id = region_id;
    let p_bucket = bucket;
    let p_put_object_storage_bucket_acl_request = put_object_storage_bucket_acl_request;

    let uri_str = format!("{}/{apiVersion}/object-storage/buckets/{regionId}/{bucket}/object-acl", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), regionId=crate::apis::urlencode(p_region_id), bucket=crate::apis::urlencode(p_bucket));
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
    req_builder = req_builder.json(&p_put_object_storage_bucket_acl_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PutObjectStorageBucketAclError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

