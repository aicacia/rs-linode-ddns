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


/// struct for typed errors of method [`get_databases_mysql_instance_ssl`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDatabasesMysqlInstanceSslError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_databases_postgresql_instance_ssl`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDatabasesPostgresqlInstanceSslError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}


/// Display the SSL CA certificate for an accessible MySQL Managed Database. The database's status needs to be `active`.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases mysql-ssl-cert 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn get_databases_mysql_instance_ssl(configuration: &configuration::Configuration, api_version: &str, instance_id: i32) -> Result<models::GetDatabasesMysqlInstanceSsl200Response, Error<GetDatabasesMysqlInstanceSslError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_instance_id = instance_id;

    let uri_str = format!("{}/{apiVersion}/databases/mysql/instances/{instanceId}/ssl", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), instanceId=p_instance_id);
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
        let entity: Option<GetDatabasesMysqlInstanceSslError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Display the SSL CA certificate for an accessible PostgreSQL Managed Database. The database's status needs to be `active`.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases postgresql-ssl-cert 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn get_databases_postgresql_instance_ssl(configuration: &configuration::Configuration, api_version: &str, instance_id: i32) -> Result<models::GetDatabasesMysqlInstanceSsl200Response, Error<GetDatabasesPostgresqlInstanceSslError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_instance_id = instance_id;

    let uri_str = format!("{}/{apiVersion}/databases/postgresql/instances/{instanceId}/ssl", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), instanceId=p_instance_id);
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
        let entity: Option<GetDatabasesPostgresqlInstanceSslError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

