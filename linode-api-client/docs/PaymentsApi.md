# \PaymentsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_payment**](PaymentsApi.md#get_payment) | **GET** /{apiVersion}/account/payments/{paymentId} | Get a payment
[**get_payments**](PaymentsApi.md#get_payments) | **GET** /{apiVersion}/account/payments | List payments
[**post_credit_card**](PaymentsApi.md#post_credit_card) | **POST** /{apiVersion}/account/credit-card | Add or edit a credit card
[**post_execute_pay_pal_payment**](PaymentsApi.md#post_execute_pay_pal_payment) | **POST** /{apiVersion}/account/payments/paypal/execute | Execute a PayPal payment
[**post_pay_pal_payment**](PaymentsApi.md#post_pay_pal_payment) | **POST** /{apiVersion}/account/payments/paypal | Stage a PayPal payment
[**post_payment**](PaymentsApi.md#post_payment) | **POST** /{apiVersion}/account/payments | Make a payment



## get_payment

> models::GetPayments200ResponseDataInner get_payment(api_version, payment_id)
Get a payment

Returns information about a specific Payment.   <<LB>>  ---   - __CLI__.      ```     linode-cli account payment-view 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**payment_id** | **i32** | The ID of the Payment to look up. | [required] |

### Return type

[**models::GetPayments200ResponseDataInner**](get_payments_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payments

> models::GetPayments200Response get_payments(api_version, page, page_size)
List payments

Returns a paginated list of Payments made on this Account.   <<LB>>  ---   - __CLI__.      ```     linode-cli account payments-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetPayments200Response**](get_payments_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_credit_card

> serde_json::Value post_credit_card(api_version, post_credit_card_request)
Add or edit a credit card

__Deprecated__ Please run [Add a payment method](https://techdocs.akamai.com/linode-api/reference/post-payment-method).  Adds a credit card Payment Method to your account and sets it as the default method.   <<LB>>  ---   - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_credit_card_request** | [**PostCreditCardRequest**](PostCreditCardRequest.md) | Update the credit card information associated with your Account. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_execute_pay_pal_payment

> serde_json::Value post_execute_pay_pal_payment(api_version, post_execute_pay_pal_payment_request)
Execute a PayPal payment

__Deprecated__ This operation is disabled and no longer accessible. PayPal can be designated as a Payment Method for automated payments using the Cloud Manager. See [Manage Payment Methods](https://www.linode.com/docs/products/platform/billing/guides/payment-methods/).   <<LB>>  ---   - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_execute_pay_pal_payment_request** | [**PostExecutePayPalPaymentRequest**](PostExecutePayPalPaymentRequest.md) | The details of the Payment to execute. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_pay_pal_payment

> models::PostPayPalPayment200Response post_pay_pal_payment(api_version, post_pay_pal_payment_request)
Stage a PayPal payment

__Deprecated__ This operation is disabled and no longer accessible. PayPal can be designated as a Payment Method for automated payments using the Cloud Manager. See [Manage Payment Methods](https://www.linode.com/docs/products/platform/billing/guides/payment-methods/).   <<LB>>  ---   - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_pay_pal_payment_request** | [**PostPayPalPaymentRequest**](PostPayPalPaymentRequest.md) | The amount of the Payment to submit via PayPal. | [required] |

### Return type

[**models::PostPayPalPayment200Response**](post_pay_pal_payment_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_payment

> models::GetPayments200ResponseDataInner post_payment(api_version, post_payment_request)
Make a payment

Makes a Payment to your Account.  - The requested amount is charged to the default Payment Method if no `payment_method_id` is specified.  - A `payment_submitted` event is generated when a payment is successfully submitted.  __Parent and child accounts__  In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:  - Child account users can't run this operation. These users don't have access to billing-related operations.   <<LB>>  ---   - __CLI__.      ```     linode-cli account payment-create \\   --usd 120.50 \\   --payment_method_id 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_payment_request** | [**PostPaymentRequest**](PostPaymentRequest.md) | Information about the Payment you are making. | [required] |

### Return type

[**models::GetPayments200ResponseDataInner**](get_payments_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

