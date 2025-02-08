# \KubeconfigsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lke_cluster_kubeconfig**](KubeconfigsApi.md#delete_lke_cluster_kubeconfig) | **DELETE** /{apiVersion}/lke/clusters/{clusterId}/kubeconfig | Delete a Kubeconfig
[**get_lke_cluster_kubeconfig**](KubeconfigsApi.md#get_lke_cluster_kubeconfig) | **GET** /{apiVersion}/lke/clusters/{clusterId}/kubeconfig | Get a Kubeconfig



## delete_lke_cluster_kubeconfig

> serde_json::Value delete_lke_cluster_kubeconfig(api_version, cluster_id)
Delete a Kubeconfig

Delete and regenerate the Kubeconfig file for a Cluster.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke kubeconfig-delete 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **i32** | ID of the Kubernetes cluster to look up. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lke_cluster_kubeconfig

> models::GetLkeClusterKubeconfig200Response get_lke_cluster_kubeconfig(api_version, cluster_id)
Get a Kubeconfig

Get the Kubeconfig file for a Cluster. Please note that it often takes 2-5 minutes before the Kubeconfig file is ready after first [creating a new cluster](https://techdocs.akamai.com/linode-api/reference/post-lke-cluster).   <<LB>>  ---   - __CLI__.      ```     linode-cli lke kubeconfig-view 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **i32** | ID of the Kubernetes cluster to look up. | [required] |

### Return type

[**models::GetLkeClusterKubeconfig200Response**](get_lke_cluster_kubeconfig_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

