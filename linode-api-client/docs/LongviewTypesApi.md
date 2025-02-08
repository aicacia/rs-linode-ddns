# \LongviewTypesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_longview_types**](LongviewTypesApi.md#get_longview_types) | **GET** /{apiVersion}/longview/types | List Longview types



## get_longview_types

> models::GetLongviewTypes200Response get_longview_types(api_version)
List Longview types

Returns Longview types and prices, including any region-specific rates.   <<LB>>  ---   - __CLI__.      ```     linode-cli longview types     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetLongviewTypes200Response**](get_longview_types_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

