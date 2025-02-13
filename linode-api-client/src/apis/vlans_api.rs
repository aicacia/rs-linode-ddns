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


/// struct for typed errors of method [`delete_vlan`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteVlanError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_vlans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVlansError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}


/// This operation deletes a VLAN. You can't delete a VLAN if it's still attached to a Linode. There are a few ways to detach it: - [Update](https://techdocs.akamai.com/linode-api/reference/put-linode-config) the active configuration profile to remove the VLAN interface, then [reboot](https://techdocs.akamai.com/linode-api/reference/post-reboot-linode-instance) the Linode. - [Create a config profile](https://techdocs.akamai.com/linode-api/reference/post-add-linode-config) without the VLAN interface, then [reboot](https://techdocs.akamai.com/linode-api/reference/post-reboot-linode-instance) the Linode into the new configuration profile. - [Delete](https://techdocs.akamai.com/linode-api/reference/delete-linode-instance) the Linode.  To run this operation, you need `read_write` grants to Linodes that use the VLAN.  A successful request triggers a `vlan_delete` event.  > 📘 > > VLANs without any attached Linodes are periodically cleaned up by the system.   <<LB>>  ---   - __CLI__.      ```     linode-cli vlans delete $regionId $label     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn delete_vlan(configuration: &configuration::Configuration, api_version: &str, label: &str, region_id: &str) -> Result<serde_json::Value, Error<DeleteVlanError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_label = label;
    let p_region_id = region_id;

    let uri_str = format!("{}/{apiVersion}/networking/vlans/{regionId}/{label}", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), label=crate::apis::urlencode(p_label), regionId=crate::apis::urlencode(p_region_id));
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
        let entity: Option<DeleteVlanError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a list of all Virtual Local Area Networks (VLANs) on your Account. VLANs provide a mechanism for secure communication between two or more Linodes that are assigned to the same VLAN and are both within the same Layer 2 broadcast domain.  VLANs are created and attached to Linodes by using the `interfaces` property for the following operations:  - [Create a Linode](https://techdocs.akamai.com/linode-api/reference/post-linode-instance) - [Create a config profile](https://techdocs.akamai.com/linode-api/reference/post-add-linode-config) - [Update a config profile](https://techdocs.akamai.com/linode-api/reference/put-linode-config)  There are several ways to detach a VLAN from a Linode:  - [Update](https://techdocs.akamai.com/linode-api/reference/put-linode-config) the active Configuration Profile to remove the VLAN Interface, then [reboot](https://techdocs.akamai.com/linode-api/reference/post-reboot-linode-instance) the Linode. - [Create a config profile](https://techdocs.akamai.com/linode-api/reference/post-add-linode-config) without the VLAN Interface, then [reboot](https://techdocs.akamai.com/linode-api/reference/post-reboot-linode-instance) the Linode into the new Configuration Profile. - [Delete](https://techdocs.akamai.com/linode-api/reference/delete-linode-instance) the Linode.  __Note__. Only Next Generation Network (NGN) data centers support VLANs. Run the [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions) operation to view the capabilities of data center regions. If a VLAN is attached to your Linode and you attempt to migrate or clone it to a non-NGN data center, the migration or cloning will not initiate. If a Linode cannot be migrated because of an incompatibility, you will be prompted to select a different data center or contact support.  __Note__. See the [VLANs Overview](https://www.linode.com/docs/products/networking/vlans/#technical-specifications) to view additional specifications and limitations.   <<LB>>  ---   - __CLI__.      ```     linode-cli vlans list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn get_vlans(configuration: &configuration::Configuration, api_version: &str, page: Option<i32>, page_size: Option<i32>) -> Result<models::GetVlans200Response, Error<GetVlansError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_page = page;
    let p_page_size = page_size;

    let uri_str = format!("{}/{apiVersion}/networking/vlans", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version));
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
        let entity: Option<GetVlansError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

