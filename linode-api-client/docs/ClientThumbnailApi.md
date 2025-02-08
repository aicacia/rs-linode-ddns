# \ClientThumbnailApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_client_thumbnail**](ClientThumbnailApi.md#get_client_thumbnail) | **GET** /{apiVersion}/account/oauth-clients/{clientId}/thumbnail | Get the OAuth client's thumbnail
[**put_client_thumbnail**](ClientThumbnailApi.md#put_client_thumbnail) | **PUT** /{apiVersion}/account/oauth-clients/{clientId}/thumbnail | Update the OAuth client's thumbnail



## get_client_thumbnail

> std::path::PathBuf get_client_thumbnail(api_version, client_id)
Get the OAuth client's thumbnail

Returns the PNG thumbnail for this OAuth Client.  This is a publicly viewable endpoint, and can be accessed without authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**client_id** | **String** | The OAuth Client ID to look up. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_client_thumbnail

> serde_json::Value put_client_thumbnail(api_version, client_id, body)
Update the OAuth client's thumbnail

Upload a thumbnail for a client you own.  You must upload a PNG image file that will be returned when the thumbnail is retrieved.  This image will be publicly viewable.   <<LB>>  ---   - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**client_id** | **String** | The OAuth Client ID to look up. | [required] |
**body** | **std::path::PathBuf** | The image to set as the thumbnail. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: image/png
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

