# \EndpointsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_object_storage_endpoints**](EndpointsApi.md#get_object_storage_endpoints) | **GET** /{apiVersion}/object-storage/endpoints | List Object Storage endpoints



## get_object_storage_endpoints

> models::GetObjectStorageEndpoints200Response get_object_storage_endpoints(api_version)
List Object Storage endpoints

Returns a paginated list of all Object Storage endpoints available in your account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetObjectStorageEndpoints200Response**](get_object_storage_endpoints_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

