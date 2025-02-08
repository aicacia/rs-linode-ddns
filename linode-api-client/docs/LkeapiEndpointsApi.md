# \LkeapiEndpointsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lke_cluster_api_endpoints**](LkeapiEndpointsApi.md#get_lke_cluster_api_endpoints) | **GET** /{apiVersion}/lke/clusters/{clusterId}/api-endpoints | List Kubernetes API endpoints



## get_lke_cluster_api_endpoints

> models::GetLkeClusterApiEndpoints200Response get_lke_cluster_api_endpoints(api_version, cluster_id)
List Kubernetes API endpoints

List the Kubernetes API server endpoints for this cluster. Please note that it often takes 2-5 minutes before the endpoint is ready after first [creating a new cluster](https://techdocs.akamai.com/linode-api/reference/post-lke-cluster).   <<LB>>  ---   - __CLI__.      ```     linode-cli lke api-endpoints-list 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **i32** | ID of the Kubernetes cluster to look up. | [required] |

### Return type

[**models::GetLkeClusterApiEndpoints200Response**](get_lke_cluster_api_endpoints_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

