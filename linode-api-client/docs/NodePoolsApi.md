# \NodePoolsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lke_node_pool**](NodePoolsApi.md#delete_lke_node_pool) | **DELETE** /{apiVersion}/lke/clusters/{clusterId}/pools/{poolId} | Delete a node pool
[**get_lke_cluster_pools**](NodePoolsApi.md#get_lke_cluster_pools) | **GET** /{apiVersion}/lke/clusters/{clusterId}/pools | List node pools
[**get_lke_node_pool**](NodePoolsApi.md#get_lke_node_pool) | **GET** /{apiVersion}/lke/clusters/{clusterId}/pools/{poolId} | Get a node pool
[**post_lke_cluster_pool_recycle**](NodePoolsApi.md#post_lke_cluster_pool_recycle) | **POST** /{apiVersion}/lke/clusters/{clusterId}/pools/{poolId}/recycle | Recycle a node pool
[**post_lke_cluster_pools**](NodePoolsApi.md#post_lke_cluster_pools) | **POST** /{apiVersion}/lke/clusters/{clusterId}/pools | Create a node pool
[**put_lke_node_pool**](NodePoolsApi.md#put_lke_node_pool) | **PUT** /{apiVersion}/lke/clusters/{clusterId}/pools/{poolId} | Update a node pool



## delete_lke_node_pool

> serde_json::Value delete_lke_node_pool(api_version, cluster_id, pool_id)
Delete a node pool

Delete a specific Node Pool from a Kubernetes cluster.  __Deleting a Node Pool is a destructive action and cannot be undone.__  Deleting a Node Pool will delete all Linodes within that Pool.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke pool-delete 12345 456     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **i32** | ID of the Kubernetes cluster to look up. | [required] |
**pool_id** | **i32** | ID of the Pool to look up. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lke_cluster_pools

> models::GetLkeClusterPools200Response get_lke_cluster_pools(api_version, cluster_id)
List node pools

Returns all active Node Pools on a Kubernetes cluster.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke pools-list 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **i32** | ID of the Kubernetes cluster to look up. | [required] |

### Return type

[**models::GetLkeClusterPools200Response**](get_lke_cluster_pools_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lke_node_pool

> models::GetLkeClusterPools200ResponseDataInner get_lke_node_pool(api_version, cluster_id, pool_id)
Get a node pool

Get a specific Node Pool by ID.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke pool-view 12345 456     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **i32** | ID of the Kubernetes cluster to look up. | [required] |
**pool_id** | **i32** | ID of the Pool to look up. | [required] |

### Return type

[**models::GetLkeClusterPools200ResponseDataInner**](get_lke_cluster_pools_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lke_cluster_pool_recycle

> serde_json::Value post_lke_cluster_pool_recycle(api_version, cluster_id, pool_id)
Recycle a node pool

Recycles a Node Pool for the designated Kubernetes Cluster. All Linodes within the Node Pool will be deleted and replaced with new Linodes on a rolling basis, which may take several minutes. Replacement Nodes are installed with the latest available patch for the Cluster's Kubernetes Version.  __Any local storage on deleted Linodes (such as `hostPath` and `emptyDir` volumes, or `local` PersistentVolumes) will be erased.__   <<LB>>  ---   - __CLI__.      ```     linode-cli lke pool-recycle 12345 456     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **i32** | ID of the Kubernetes cluster this Node Pool is attached to. | [required] |
**pool_id** | **i32** | ID of the Node Pool to be recycled. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lke_cluster_pools

> models::GetLkeClusterPools200ResponseDataInner post_lke_cluster_pools(api_version, cluster_id, post_lke_cluster_pools_request)
Create a node pool

Creates a new Node Pool for the designated Kubernetes cluster.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke pool-create 12345 \\   --type g6-standard-4 \\   --count 6 \\   --tags example-tag \\   --autoscaler.enabled true \\   --autoscaler.max 12 \\   --autoscaler.min 3 \\   --labels '{ \"example.com/my-app\":\"team1\" }' \\   --taints.effect \"NoSchedule\" \\   --taints.key \"example.com/my-app\" \\   --taints.value \"teamA\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **i32** | ID of the Kubernetes cluster to look up. | [required] |
**post_lke_cluster_pools_request** | [**PostLkeClusterPoolsRequest**](PostLkeClusterPoolsRequest.md) | Configuration for the Node Pool. | [required] |

### Return type

[**models::GetLkeClusterPools200ResponseDataInner**](get_lke_cluster_pools_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lke_node_pool

> models::GetLkeClusterPools200ResponseDataInner put_lke_node_pool(api_version, cluster_id, pool_id, put_lke_node_pool_request)
Update a node pool

Updates a node pool's count, labels and taints, and autoscaler configuration.  Linodes are created or deleted to match changes to the Node Pool's count.  Specifying labels or taints on update overwrites any previous values, and updates existing nodes with the new values without a recycle.  __Any local storage on deleted Linodes (such as `hostPath` and `emptyDir` volumes, or `local` PersistentVolumes) will be erased.__   <<LB>>  ---   - __CLI__.      ```     linode-cli lke pool-update 12345 456 \\   --count 6 \\   --autoscaler.enabled true \\   --autoscaler.max 12 \\   --autoscaler.min 3 \\   --labels '{ \"example.com/my-app\":\"team1\", \"env\":\"staging\" }' \\   --taints.effect \"NoSchedule\" \\   --taints.key \"example.com/my-app\" \\   --taints.value \"teamA\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **i32** | ID of the Kubernetes cluster to look up. | [required] |
**pool_id** | **i32** | ID of the Pool to look up. | [required] |
**put_lke_node_pool_request** | Option<[**PutLkeNodePoolRequest**](PutLkeNodePoolRequest.md)> | The fields to update. |  |

### Return type

[**models::GetLkeClusterPools200ResponseDataInner**](get_lke_cluster_pools_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

