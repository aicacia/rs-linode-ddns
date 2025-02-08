# \AccountSettingsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account_settings**](AccountSettingsApi.md#get_account_settings) | **GET** /{apiVersion}/account/settings | Get account settings
[**put_account_settings**](AccountSettingsApi.md#put_account_settings) | **PUT** /{apiVersion}/account/settings | Update account settings



## get_account_settings

> models::GetAccountSettings200Response get_account_settings(api_version)
Get account settings

Returns information related to your Account settings: Managed service subscription, Longview subscription, and network helper.   <<LB>>  ---   - __CLI__.      ```     linode-cli account settings     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetAccountSettings200Response**](get_account_settings_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_account_settings

> models::GetAccountSettings200Response put_account_settings(api_version, put_account_settings_request)
Update account settings

Updates your account settings. For a Longview subscription plan, see [Update a Longview plan](https://techdocs.akamai.com/linode-api/reference/put-longview-plan).   <<LB>>  ---   - __CLI__.      ```     linode-cli account settings-update \\   --network_helper false     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**put_account_settings_request** | [**PutAccountSettingsRequest**](PutAccountSettingsRequest.md) | Update Account settings information. | [required] |

### Return type

[**models::GetAccountSettings200Response**](get_account_settings_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

