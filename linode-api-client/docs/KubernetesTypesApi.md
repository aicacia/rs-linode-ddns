# \KubernetesTypesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lke_types**](KubernetesTypesApi.md#get_lke_types) | **GET** /{apiVersion}/lke/types | List Kubernetes types



## get_lke_types

> models::GetLkeTypes200Response get_lke_types(api_version)
List Kubernetes types

Returns Kubernetes types and prices, including any region-specific rates.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke types     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetLkeTypes200Response**](get_lke_types_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

