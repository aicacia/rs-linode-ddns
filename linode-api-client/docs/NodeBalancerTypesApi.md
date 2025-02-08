# \NodeBalancerTypesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_node_balancer_types**](NodeBalancerTypesApi.md#get_node_balancer_types) | **GET** /{apiVersion}/nodebalancers/types | List NodeBalancer types



## get_node_balancer_types

> models::GetNodeBalancerTypes200Response get_node_balancer_types(api_version)
List NodeBalancer types

Returns NodeBalancer types and prices, including any region-specific rates.   <<LB>>  ---   - __CLI__.      ```     linode-cli nodebalancers types     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetNodeBalancerTypes200Response**](get_node_balancer_types_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

