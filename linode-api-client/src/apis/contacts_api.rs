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


/// struct for typed errors of method [`delete_managed_contact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteManagedContactError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_managed_contact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetManagedContactError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_managed_contacts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetManagedContactsError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_managed_contact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostManagedContactError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_managed_contact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutManagedContactError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}


/// Deletes a Managed Contact.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed contact-delete 567     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn delete_managed_contact(configuration: &configuration::Configuration, api_version: &str, contact_id: i32) -> Result<serde_json::Value, Error<DeleteManagedContactError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_contact_id = contact_id;

    let uri_str = format!("{}/{apiVersion}/managed/contacts/{contactId}", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), contactId=p_contact_id);
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
        let entity: Option<DeleteManagedContactError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a single Managed Contact.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed contact-view 567     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn get_managed_contact(configuration: &configuration::Configuration, api_version: &str, contact_id: i32) -> Result<models::GetManagedContacts200ResponseDataInner, Error<GetManagedContactError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_contact_id = contact_id;

    let uri_str = format!("{}/{apiVersion}/managed/contacts/{contactId}", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), contactId=p_contact_id);
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
        let entity: Option<GetManagedContactError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a paginated list of Managed Contacts on your Account.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed contacts-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn get_managed_contacts(configuration: &configuration::Configuration, api_version: &str, page: Option<i32>, page_size: Option<i32>) -> Result<models::GetManagedContacts200Response, Error<GetManagedContactsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_page = page;
    let p_page_size = page_size;

    let uri_str = format!("{}/{apiVersion}/managed/contacts", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("page_size", &param_value.to_string())]);
    }
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
        let entity: Option<GetManagedContactsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Creates a Managed Contact.  A Managed Contact is someone Linode special forces can contact in the course of attempting to resolve an issue with a Managed Service.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed contact-create \\   --name \"John Doe\" \\   --email \"john.doe@example.org\" \\   --phone.primary \"123-456-7890\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn post_managed_contact(configuration: &configuration::Configuration, api_version: &str, post_managed_contact_request: Option<models::PostManagedContactRequest>) -> Result<models::GetManagedContacts200ResponseDataInner, Error<PostManagedContactError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_post_managed_contact_request = post_managed_contact_request;

    let uri_str = format!("{}/{apiVersion}/managed/contacts", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version));
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
    req_builder = req_builder.json(&p_post_managed_contact_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PostManagedContactError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Updates information about a Managed Contact. This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed contact-update 567 \\   --name \"John Doe\" \\   --email \"john.doe@example.org\" \\   --phone.primary \"123-456-7890\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn put_managed_contact(configuration: &configuration::Configuration, api_version: &str, contact_id: i32, post_managed_contact_request: models::PostManagedContactRequest) -> Result<models::GetManagedContacts200ResponseDataInner, Error<PutManagedContactError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_contact_id = contact_id;
    let p_post_managed_contact_request = post_managed_contact_request;

    let uri_str = format!("{}/{apiVersion}/managed/contacts/{contactId}", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), contactId=p_contact_id);
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
    req_builder = req_builder.json(&p_post_managed_contact_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PutManagedContactError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

