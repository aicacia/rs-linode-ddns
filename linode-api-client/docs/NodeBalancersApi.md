# \NodeBalancersApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_node_balancer**](NodeBalancersApi.md#delete_node_balancer) | **DELETE** /{apiVersion}/nodebalancers/{nodeBalancerId} | Delete a NodeBalancer
[**get_linode_node_balancers**](NodeBalancersApi.md#get_linode_node_balancers) | **GET** /{apiVersion}/linode/instances/{linodeId}/nodebalancers | List Linode NodeBalancers
[**get_node_balancer**](NodeBalancersApi.md#get_node_balancer) | **GET** /{apiVersion}/nodebalancers/{nodeBalancerId} | Get a NodeBalancer
[**get_node_balancers**](NodeBalancersApi.md#get_node_balancers) | **GET** /{apiVersion}/nodebalancers | List NodeBalancers
[**post_node_balancer**](NodeBalancersApi.md#post_node_balancer) | **POST** /{apiVersion}/nodebalancers | Create a NodeBalancer
[**put_node_balancer**](NodeBalancersApi.md#put_node_balancer) | **PUT** /{apiVersion}/nodebalancers/{nodeBalancerId} | Update a NodeBalancer



## delete_node_balancer

> serde_json::Value delete_node_balancer(api_version, node_balancer_id)
Delete a NodeBalancer

Deletes a NodeBalancer.  __This is a destructive action and cannot be undone.__  Deleting a NodeBalancer will also delete all associated Configs and Nodes, although the backend servers represented by the Nodes will not be changed or removed. Deleting a NodeBalancer will cause you to lose access to the IP Addresses assigned to this NodeBalancer.   <<LB>>  ---   - __CLI__.      ```     linode-cli nodebalancers delete 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     nodebalancers:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**node_balancer_id** | **i32** | The ID of the NodeBalancer to access. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_node_balancers

> models::GetLinodeNodeBalancers200Response get_linode_node_balancers(api_version, linode_id)
List Linode NodeBalancers

Returns a list of NodeBalancers that are assigned to this Linode and readable by the requesting User.  Read permission to a NodeBalancer can be given to a User by accessing the [Update a user's grants](https://techdocs.akamai.com/linode-api/reference/put-user-grants) operation.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes nodebalancers 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up. | [required] |

### Return type

[**models::GetLinodeNodeBalancers200Response**](get_linode_node_balancers_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_node_balancer

> models::NodeBalancer3 get_node_balancer(api_version, node_balancer_id)
Get a NodeBalancer

Returns a single NodeBalancer you can access.   <<LB>>  ---   - __CLI__.      ```     linode-cli nodebalancers view 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     nodebalancers:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**node_balancer_id** | **i32** | The ID of the NodeBalancer to access. | [required] |

### Return type

[**models::NodeBalancer3**](NodeBalancer_3.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_node_balancers

> models::GetNodeBalancers200Response get_node_balancers(api_version, page, page_size)
List NodeBalancers

Returns a paginated list of NodeBalancers you have access to.   <<LB>>  ---   - __CLI__.      ```     linode-cli nodebalancers list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     nodebalancers:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetNodeBalancers200Response**](get_node_balancers_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_node_balancer

> models::NodeBalancer2 post_node_balancer(api_version, post_node_balancer_request)
Create a NodeBalancer

Creates a NodeBalancer in the requested Region. Only available in [regions](https://techdocs.akamai.com/linode-api/reference/get-regions) with \"NodeBalancers\" in their `capabilities`.  NodeBalancers require a port config with at least one backend node to start serving requests.  When using the Linode CLI to create a NodeBalancer, first create a NodeBalancer without any configs. Then, create configs and nodes for that NodeBalancer with the respective [Create a config](https://techdocs.akamai.com/linode-api/reference/post-node-balancer-config) and [Create a node](https://techdocs.akamai.com/linode-api/reference/post-node-balancer-node) operations.   <<LB>>  ---   - __CLI__.      ```     linode-cli nodebalancers create \\   --region us-east \\   --label balancer12345 \\   --client_conn_throttle 0     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     nodebalancers:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_node_balancer_request** | [**PostNodeBalancerRequest**](PostNodeBalancerRequest.md) | Information about the NodeBalancer to create. | [required] |

### Return type

[**models::NodeBalancer2**](NodeBalancer_2.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_node_balancer

> models::NodeBalancer5 put_node_balancer(api_version, node_balancer_id, node_balancer4)
Update a NodeBalancer

Updates information about a NodeBalancer you can access.   <<LB>>  ---   - __CLI__.      ```     linode-cli nodebalancers update 12345 \\   --label balancer12345 \\   --client_conn_throttle 0     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     nodebalancers:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**node_balancer_id** | **i32** | The ID of the NodeBalancer to access. | [required] |
**node_balancer4** | [**NodeBalancer4**](NodeBalancer4.md) | The information to update. | [required] |

### Return type

[**models::NodeBalancer5**](NodeBalancer_5.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

