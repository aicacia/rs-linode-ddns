# \AccountApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account**](AccountApi.md#get_account) | **GET** /{apiVersion}/account | Get your account
[**get_account_availability**](AccountApi.md#get_account_availability) | **GET** /{apiVersion}/account/availability/{id} | Get a region's service availability
[**post_cancel_account**](AccountApi.md#post_cancel_account) | **POST** /{apiVersion}/account/cancel | Cancel your account
[**put_account**](AccountApi.md#put_account) | **PUT** /{apiVersion}/account | Update your account



## get_account

> models::GetAccount200Response get_account(api_version)
Get your account

Returns the contact and billing information related to your account.   <<LB>>  ---   - __CLI__.      ```     linode-cli account view     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetAccount200Response**](get_account_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_availability

> models::GetAvailability200ResponseAllOfDataInner get_account_availability(api_version, id)
Get a region's service availability

View the available services for your account, in a specific region.  > 📘 > > Only account users with _unrestricted_ access can run this operation.   <<LB>>  ---   - __CLI__.      ```     linode-cli account get-account-availability us-east     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**id** | **String** | The slug for the applicable data center. Run the [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions) operation to view the slug for each data center. | [required] |

### Return type

[**models::GetAvailability200ResponseAllOfDataInner**](get_availability_200_response_allOf_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_cancel_account

> models::PostCancelAccount200Response post_cancel_account(api_version, post_cancel_account_request)
Cancel your account

Cancels an active account. Akamai attempts to charge the credit card on file for any remaining balance. An error occurs if this charge fails.  > 📘 > > Only account users with _unrestricted_ access can run this operation.  __Parent and child accounts__ In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:  - A child account user can't cancel a child account.  - You can't cancel a parent account if it has an active child account.  - You need to work with your Akamai account team to dissolve any parent-child account relationships before you can fully cancel a child or parent account.   <<LB>>  ---   - __CLI__.      ```     linode-cli account cancel \\   --comments \"I'm consolidating my accounts\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_cancel_account_request** | [**PostCancelAccountRequest**](PostCancelAccountRequest.md) | Supply a comment stating the reason that you are cancelling your account. | [required] |

### Return type

[**models::PostCancelAccount200Response**](post_cancel_account_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_account

> models::GetAccount200Response put_account(api_version, put_account_request)
Update your account

Updates contact and billing information related to your account. If you exclude any properties from the request, the operation leaves them unchanged.  > 📘 > > When updating an account's `country` to `US`, you'll get an error if the account's `zip` is not a valid US zip code.  **Parent and child accounts**  In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:  - You can't change the `company` for a parent account. Akamai uses this value to set the name for a child account parent user (proxy user) on any child account.  - Child account users can't run this operation. These users don't have access to billing-related operations.   <<LB>>  ---   - __CLI__.      ```     linode-cli account update \\   --address_1 \"123 Main St.\" \\   --address_2 \"Suite 101\" \\   --city Philadelphia \\   --company My Company \\ LLC \\   --country US \\   --email jsmith@mycompany.com \\   --first_name John \\   --last_name Smith \\   --phone 555-555-1212 \\   --state PA \\   --tax_id ATU99999999 \\   --zip 19102     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**put_account_request** | [**PutAccountRequest**](PutAccountRequest.md) | Updated contact and billing information. | [required] |

### Return type

[**models::GetAccount200Response**](get_account_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

