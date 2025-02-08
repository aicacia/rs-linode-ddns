# \PaymentMethodsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_payment_method**](PaymentMethodsApi.md#delete_payment_method) | **DELETE** /{apiVersion}/account/payment-methods/{paymentMethodId} | Delete a payment method
[**get_payment_method**](PaymentMethodsApi.md#get_payment_method) | **GET** /{apiVersion}/account/payment-methods/{paymentMethodId} | Get a payment method
[**get_payment_methods**](PaymentMethodsApi.md#get_payment_methods) | **GET** /{apiVersion}/account/payment-methods | List payment methods
[**post_make_payment_method_default**](PaymentMethodsApi.md#post_make_payment_method_default) | **POST** /{apiVersion}/account/payment-methods/{paymentMethodId}/make-default | Set a default payment method
[**post_payment_method**](PaymentMethodsApi.md#post_payment_method) | **POST** /{apiVersion}/account/payment-methods | Add a payment method



## delete_payment_method

> serde_json::Value delete_payment_method(api_version, payment_method_id)
Delete a payment method

Deactivate the specified Payment Method.  The default Payment Method can not be deleted. To add a new default Payment Method, run the [Add a payment method](https://techdocs.akamai.com/linode-api/reference/post-payment-method) operation. To designate an existing Payment Method as the default method, run the [Set a default payment method](https://techdocs.akamai.com/linode-api/reference/post-make-payment-method-default) operation.   <<LB>>  ---   - __CLI__.      ```     linode-cli payment-methods delete 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**payment_method_id** | **i32** | The ID of the Payment Method to look up. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_method

> models::GetPaymentMethods200ResponseDataInner get_payment_method(api_version, payment_method_id)
Get a payment method

View the details of the specified Payment Method.   <<LB>>  ---   - __CLI__.      ```     linode-cli payment-methods view 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**payment_method_id** | **i32** | The ID of the Payment Method to look up. | [required] |

### Return type

[**models::GetPaymentMethods200ResponseDataInner**](get_payment_methods_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_methods

> models::GetPaymentMethods200Response get_payment_methods(api_version, page, page_size)
List payment methods

Returns a paginated list of Payment Methods for this Account.   <<LB>>  ---   - __CLI__.      ```     linode-cli payment-methods list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetPaymentMethods200Response**](get_payment_methods_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_make_payment_method_default

> serde_json::Value post_make_payment_method_default(api_version, payment_method_id)
Set a default payment method

Make the specified Payment Method the default method for automatically processing payments. Removes the default status from any other Payment Method.  __Parent and child accounts__  In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:  - Child account users can't run this operation. These users don't have access to billing-related operations.   <<LB>>  ---   - __CLI__.      ```     linode-cli payment-methods default 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**payment_method_id** | **i32** | The ID of the Payment Method to make default. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_payment_method

> serde_json::Value post_payment_method(api_version, post_payment_method_request)
Add a payment method

Adds a Payment Method to your Account with the option to set it as the default method.  - Adding a default Payment Method removes the default status from any other Payment Method.  - An Account can have up to 6 active Payment Methods.  - Up to 60 Payment Methods can be added each day.  - Prior to adding a Payment Method, ensure that your billing address information is up-to-date with a valid `zip` by running the [Update your account](https://techdocs.akamai.com/linode-api/reference/put-account) operation.  - A `payment_method_add` event is generated when a payment is successfully submitted.  __Parent and child accounts__  In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:  - Child account users can't run this operation. These users don't have access to billing-related operations.   <<LB>>  ---   - __CLI__.      ```     linode-cli payment-methods add \\   --type credit_card \\   --is_default true \\   --data.card_number 4111111111111111 \\   --data.expiry_month 11 \\   --data.expiry_year 2020 \\   --data.cvv 111     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_payment_method_request** | [**PostPaymentMethodRequest**](PostPaymentMethodRequest.md) | The details of the Payment Method to add. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

