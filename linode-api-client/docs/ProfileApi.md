# \ProfileApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_profile**](ProfileApi.md#get_profile) | **GET** /{apiVersion}/profile | Get a profile
[**put_profile**](ProfileApi.md#put_profile) | **PUT** /{apiVersion}/profile | Update a profile



## get_profile

> models::GetProfile200Response get_profile(api_version)
Get a profile

Returns information about the current User. This can be used to see who is acting in applications where more than one token is managed. For example, in third-party OAuth applications.  This operation is always accessible, no matter what OAuth scopes the acting token has.   <<LB>>  ---   - __CLI__.      ```     linode-cli profile view     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetProfile200Response**](get_profile_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_profile

> models::GetProfile200Response put_profile(api_version, put_profile_request)
Update a profile

Update information in your Profile.  This operation requires the `account:read_write` OAuth Scope.  __Parent and child accounts__  In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:  - You can't edit the `email` for a child account parent user (proxy user). This value is fixed and set when you provision this environment.   <<LB>>  ---   - __CLI__.      ```     linode-cli profile update \\   --email example-user@gmail.com \\   --timezone US/Eastern \\   --email_notifications true \\   --list_auth_method keys_only \\   --two_factor_auth true \\   --restricted false     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**put_profile_request** | [**PutProfileRequest**](PutProfileRequest.md) | The fields to update. | [required] |

### Return type

[**models::GetProfile200Response**](get_profile_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

