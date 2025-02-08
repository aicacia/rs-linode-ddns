# \NodesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lke_cluster_node**](NodesApi.md#delete_lke_cluster_node) | **DELETE** /{apiVersion}/lke/clusters/{clusterId}/nodes/{nodeId} | Delete a node
[**delete_node_balancer_config_node**](NodesApi.md#delete_node_balancer_config_node) | **DELETE** /{apiVersion}/nodebalancers/{nodeBalancerId}/configs/{configId}/nodes/{nodeId} | Delete a NodeBalancer's node
[**get_lke_cluster_node**](NodesApi.md#get_lke_cluster_node) | **GET** /{apiVersion}/lke/clusters/{clusterId}/nodes/{nodeId} | Get a node
[**get_node_balancer_config_nodes**](NodesApi.md#get_node_balancer_config_nodes) | **GET** /{apiVersion}/nodebalancers/{nodeBalancerId}/configs/{configId}/nodes | List nodes
[**get_node_balancer_node**](NodesApi.md#get_node_balancer_node) | **GET** /{apiVersion}/nodebalancers/{nodeBalancerId}/configs/{configId}/nodes/{nodeId} | Get a NodeBalancer's node
[**post_lke_cluster_node_recycle**](NodesApi.md#post_lke_cluster_node_recycle) | **POST** /{apiVersion}/lke/clusters/{clusterId}/nodes/{nodeId}/recycle | Recycle a node
[**post_node_balancer_node**](NodesApi.md#post_node_balancer_node) | **POST** /{apiVersion}/nodebalancers/{nodeBalancerId}/configs/{configId}/nodes | Create a node
[**put_node_balancer_node**](NodesApi.md#put_node_balancer_node) | **PUT** /{apiVersion}/nodebalancers/{nodeBalancerId}/configs/{configId}/nodes/{nodeId} | Update a node



## delete_lke_cluster_node

> serde_json::Value delete_lke_cluster_node(api_version, cluster_id, node_id)
Delete a node

Deletes a specific Node from a Node Pool.  __Deleting a Node is a destructive action and cannot be undone.__  Deleting a Node will reduce the size of the Node Pool it belongs to.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke node-delete 12345 12345-6aa78910bc     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **i32** | ID of the Kubernetes cluster containing the Node. | [required] |
**node_id** | **String** | The ID of the Node to access. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_node_balancer_config_node

> serde_json::Value delete_node_balancer_config_node(api_version, node_balancer_id, config_id, node_id)
Delete a NodeBalancer's node

Deletes a Node from this Config. This backend will no longer receive traffic for the configured port of this NodeBalancer.  This does not change or remove the Linode whose address was used in the creation of this Node.   <<LB>>  ---   - __CLI__.      ```     linode-cli nodebalancers node-delete \\   12345 4567 54321     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     nodebalancers:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**node_balancer_id** | **i32** | The ID of the NodeBalancer to access. | [required] |
**config_id** | **i32** | The ID of the Config to access. | [required] |
**node_id** | **String** | The ID of the Node to access. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lke_cluster_node

> models::GetLkeClusterNode200Response get_lke_cluster_node(api_version, cluster_id, node_id)
Get a node

Returns the values for a specified node object.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke node-view 123456 12345-6aa78910bc     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **i32** | ID of the Kubernetes cluster containing the Node. | [required] |
**node_id** | **String** | The ID of the Node to access. | [required] |

### Return type

[**models::GetLkeClusterNode200Response**](get_lke_cluster_node_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_node_balancer_config_nodes

> models::GetNodeBalancerConfigNodes200Response get_node_balancer_config_nodes(api_version, node_balancer_id, config_id, page, page_size)
List nodes

Returns a paginated list of NodeBalancer nodes associated with this Config. These are the backends that will be sent traffic for this port.   <<LB>>  ---   - __CLI__.      ```     linode-cli nodebalancers nodes-list 12345 4567     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     nodebalancers:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**node_balancer_id** | **i32** | The ID of the NodeBalancer to access. | [required] |
**config_id** | **i32** | The ID of the NodeBalancer config to access. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetNodeBalancerConfigNodes200Response**](get_node_balancer_config_nodes_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_node_balancer_node

> models::GetNodeBalancerConfigNodes200ResponseDataInner get_node_balancer_node(api_version, node_balancer_id, config_id, node_id)
Get a NodeBalancer's node

Returns information about a single Node, a backend for this NodeBalancer's configured port.   <<LB>>  ---   - __CLI__.      ```     linode-cli nodebalancers node-view 12345 4567 54321     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     nodebalancers:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**node_balancer_id** | **i32** | The ID of the NodeBalancer to access. | [required] |
**config_id** | **i32** | The ID of the Config to access. | [required] |
**node_id** | **String** | The ID of the Node to access. | [required] |

### Return type

[**models::GetNodeBalancerConfigNodes200ResponseDataInner**](get_node_balancer_config_nodes_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lke_cluster_node_recycle

> serde_json::Value post_lke_cluster_node_recycle(api_version, cluster_id, node_id)
Recycle a node

Recycles an individual Node in the designated Kubernetes Cluster. The Node will be deleted and replaced with a new Linode, which may take a few minutes. Replacement Nodes are installed with the latest available patch for the Cluster's Kubernetes Version.  __Any local storage on deleted Linodes (such as `hostPath` and `emptyDir` volumes, or `local` PersistentVolumes) will be erased.__   <<LB>>  ---   - __CLI__.      ```     linode-cli lke node-recycle 12345 12345-6aa78910bc     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **i32** | ID of the Kubernetes cluster containing the Node. | [required] |
**node_id** | **String** | ID of the Node to be recycled. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_node_balancer_node

> models::GetNodeBalancerConfigNodes200ResponseDataInner post_node_balancer_node(api_version, node_balancer_id, config_id, post_node_balancer_node_request)
Create a node

Creates a NodeBalancer node, a backend that can accept traffic for this NodeBalancer Config. Nodes are routed requests on the configured port based on their status.   <<LB>>  ---   - __CLI: TCP, HTTP, HTTPS__.      ```     linode-cli nodebalancers node-create \\   12345 4567 \\   --address 192.168.210.120:80 \\   --label node54321 \\   --weight 50 \\   --mode accept     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     nodebalancers:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**node_balancer_id** | **i32** | The ID of the NodeBalancer to access. | [required] |
**config_id** | **i32** | The ID of the NodeBalancer config to access. | [required] |
**post_node_balancer_node_request** | [**PostNodeBalancerNodeRequest**](PostNodeBalancerNodeRequest.md) | Information about the Node to create. | [required] |

### Return type

[**models::GetNodeBalancerConfigNodes200ResponseDataInner**](get_node_balancer_config_nodes_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_node_balancer_node

> models::GetNodeBalancerConfigNodes200ResponseDataInner put_node_balancer_node(api_version, node_balancer_id, config_id, node_id, get_node_balancer_config_nodes200_response_data_inner)
Update a node

Updates information about a Node, a backend for this NodeBalancer's configured port.   <<LB>>  ---   - __CLI: TCP, HTTP, HTTPS__.      ```     linode-cli nodebalancers node-update \\   12345 4567 54321 \\   --address 192.168.210.120:80 \\   --label node54321 \\   --weight 50 \\   --mode accept     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     nodebalancers:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**node_balancer_id** | **i32** | The ID of the NodeBalancer to access. | [required] |
**config_id** | **i32** | The ID of the Config to access. | [required] |
**node_id** | **String** | The ID of the Node to access. | [required] |
**get_node_balancer_config_nodes200_response_data_inner** | [**GetNodeBalancerConfigNodes200ResponseDataInner**](GetNodeBalancerConfigNodes200ResponseDataInner.md) | The fields to update. | [required] |

### Return type

[**models::GetNodeBalancerConfigNodes200ResponseDataInner**](get_node_balancer_config_nodes_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

