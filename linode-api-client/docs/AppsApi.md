# \AppsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_profile_app**](AppsApi.md#delete_profile_app) | **DELETE** /{apiVersion}/profile/apps/{appId} | Revoke app access
[**get_profile_app**](AppsApi.md#get_profile_app) | **GET** /{apiVersion}/profile/apps/{appId} | Get an authorized app
[**get_profile_apps**](AppsApi.md#get_profile_apps) | **GET** /{apiVersion}/profile/apps | List authorized apps



## delete_profile_app

> serde_json::Value delete_profile_app(api_version, app_id)
Revoke app access

Expires this app token. This token may no longer be used to access your Account.   <<LB>>  ---   - __CLI__.      ```     linode-cli profile app-delete 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**app_id** | **i32** | The authorized app ID to manage. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profile_app

> models::GetProfileApps200ResponseDataInner get_profile_app(api_version, app_id)
Get an authorized app

Returns information about a single app you've authorized to access your Account.   <<LB>>  ---   - __CLI__.      ```     linode-cli profile app-view 1234     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**app_id** | **i32** | The authorized app ID to manage. | [required] |

### Return type

[**models::GetProfileApps200ResponseDataInner**](get_profile_apps_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profile_apps

> models::GetProfileApps200Response get_profile_apps(api_version, page, page_size)
List authorized apps

This is a collection of OAuth apps that you've given access to your Account, and includes the level of access granted.   <<LB>>  ---   - __CLI__.      ```     linode-cli profile apps-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetProfileApps200Response**](get_profile_apps_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

