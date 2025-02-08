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


/// struct for typed errors of method [`get_region`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRegionError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_region_availability`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRegionAvailabilityError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_regions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRegionsError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_regions_availability`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRegionsAvailabilityError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}


/// Returns a single Region.   <<LB>>  ---   - __CLI__.      ```     linode-cli regions view us-east     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)
pub async fn get_region(configuration: &configuration::Configuration, api_version: &str, region_id: &str) -> Result<models::GetRegions200ResponseDataInner, Error<GetRegionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_region_id = region_id;

    let uri_str = format!("{}/{apiVersion}/regions/{regionId}", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), regionId=crate::apis::urlencode(p_region_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetRegionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns availability data for a single Region.   <<LB>>  ---   - __CLI__.      ```     linode-cli regions view-avail us-east     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)
pub async fn get_region_availability(configuration: &configuration::Configuration, api_version: &str, region_id: &str) -> Result<Vec<models::GetRegionsAvailability200ResponseAllOfDataInner>, Error<GetRegionAvailabilityError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_region_id = region_id;

    let uri_str = format!("{}/{apiVersion}/regions/{regionId}/availability", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), regionId=crate::apis::urlencode(p_region_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetRegionAvailabilityError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Lists the Regions available for Linode services. Not all services are guaranteed to be available in all Regions.   <<LB>>  ---   - __CLI__.      ```     linode-cli regions list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)
pub async fn get_regions(configuration: &configuration::Configuration, api_version: &str) -> Result<models::GetRegions200Response, Error<GetRegionsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;

    let uri_str = format!("{}/{apiVersion}/regions", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetRegionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns availability data for all Regions.  Currently, this operation returns availability of select premium and GPU plans for select regions.   <<LB>>  ---   - __CLI__.      ```     linode-cli regions list-avail     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)
pub async fn get_regions_availability(configuration: &configuration::Configuration, api_version: &str) -> Result<models::GetRegionsAvailability200Response, Error<GetRegionsAvailabilityError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;

    let uri_str = format!("{}/{apiVersion}/regions/availability", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetRegionsAvailabilityError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

