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


/// struct for typed errors of method [`get_payment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPaymentError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_payments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPaymentsError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_credit_card`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCreditCardError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_execute_pay_pal_payment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostExecutePayPalPaymentError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_pay_pal_payment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostPayPalPaymentError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_payment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostPaymentError {
    DefaultResponse(models::GetAccountDefaultResponse),
    UnknownValue(serde_json::Value),
}


/// Returns information about a specific Payment.   <<LB>>  ---   - __CLI__.      ```     linode-cli account payment-view 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn get_payment(configuration: &configuration::Configuration, api_version: &str, payment_id: i32) -> Result<models::GetPayments200ResponseDataInner, Error<GetPaymentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_payment_id = payment_id;

    let uri_str = format!("{}/{apiVersion}/account/payments/{paymentId}", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version), paymentId=p_payment_id);
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
        let entity: Option<GetPaymentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a paginated list of Payments made on this Account.   <<LB>>  ---   - __CLI__.      ```     linode-cli account payments-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn get_payments(configuration: &configuration::Configuration, api_version: &str, page: Option<i32>, page_size: Option<i32>) -> Result<models::GetPayments200Response, Error<GetPaymentsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_page = page;
    let p_page_size = page_size;

    let uri_str = format!("{}/{apiVersion}/account/payments", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version));
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
        let entity: Option<GetPaymentsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// __Deprecated__ Please run [Add a payment method](https://techdocs.akamai.com/linode-api/reference/post-payment-method).  Adds a credit card Payment Method to your account and sets it as the default method.   <<LB>>  ---   - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn post_credit_card(configuration: &configuration::Configuration, api_version: &str, post_credit_card_request: models::PostCreditCardRequest) -> Result<serde_json::Value, Error<PostCreditCardError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_post_credit_card_request = post_credit_card_request;

    let uri_str = format!("{}/{apiVersion}/account/credit-card", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version));
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
    req_builder = req_builder.json(&p_post_credit_card_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PostCreditCardError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// __Deprecated__ This operation is disabled and no longer accessible. PayPal can be designated as a Payment Method for automated payments using the Cloud Manager. See [Manage Payment Methods](https://www.linode.com/docs/products/platform/billing/guides/payment-methods/).   <<LB>>  ---   - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn post_execute_pay_pal_payment(configuration: &configuration::Configuration, api_version: &str, post_execute_pay_pal_payment_request: models::PostExecutePayPalPaymentRequest) -> Result<serde_json::Value, Error<PostExecutePayPalPaymentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_post_execute_pay_pal_payment_request = post_execute_pay_pal_payment_request;

    let uri_str = format!("{}/{apiVersion}/account/payments/paypal/execute", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version));
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
    req_builder = req_builder.json(&p_post_execute_pay_pal_payment_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PostExecutePayPalPaymentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// __Deprecated__ This operation is disabled and no longer accessible. PayPal can be designated as a Payment Method for automated payments using the Cloud Manager. See [Manage Payment Methods](https://www.linode.com/docs/products/platform/billing/guides/payment-methods/).   <<LB>>  ---   - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn post_pay_pal_payment(configuration: &configuration::Configuration, api_version: &str, post_pay_pal_payment_request: models::PostPayPalPaymentRequest) -> Result<models::PostPayPalPayment200Response, Error<PostPayPalPaymentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_post_pay_pal_payment_request = post_pay_pal_payment_request;

    let uri_str = format!("{}/{apiVersion}/account/payments/paypal", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version));
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
    req_builder = req_builder.json(&p_post_pay_pal_payment_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PostPayPalPaymentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Makes a Payment to your Account.  - The requested amount is charged to the default Payment Method if no `payment_method_id` is specified.  - A `payment_submitted` event is generated when a payment is successfully submitted.  __Parent and child accounts__  In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:  - Child account users can't run this operation. These users don't have access to billing-related operations.   <<LB>>  ---   - __CLI__.      ```     linode-cli account payment-create \\   --usd 120.50 \\   --payment_method_id 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
pub async fn post_payment(configuration: &configuration::Configuration, api_version: &str, post_payment_request: models::PostPaymentRequest) -> Result<models::GetPayments200ResponseDataInner, Error<PostPaymentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_api_version = api_version;
    let p_post_payment_request = post_payment_request;

    let uri_str = format!("{}/{apiVersion}/account/payments", configuration.base_path, apiVersion=crate::apis::urlencode(p_api_version));
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
    req_builder = req_builder.json(&p_post_payment_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PostPaymentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

