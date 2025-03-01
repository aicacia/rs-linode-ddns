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


/// struct for typed errors of method [`get_profile_grants`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetProfileGrantsError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}


/// This returns a GrantsResponse describing what the acting User has been granted access to.  For unrestricted users, this will return a  204 and no body because unrestricted users have access to everything without grants.  This will not return information about entities you do not have access to.  This operation is useful when writing third-party OAuth applications to see what options you should present to the acting User.  For example, if they do not have `global.add_linodes`, you might not display a button to deploy a new Linode.  Any client may run this operation; no OAuth scopes are required.
pub async fn get_profile_grants(configuration: &configuration::Configuration, api_version: &str) -> Result<models::GetUserGrants200Response, Error<GetProfileGrantsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;

    let uri_str = format!("{}/{apiVersion}/profile/grants", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version));
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
        let entity: Option<GetProfileGrantsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

