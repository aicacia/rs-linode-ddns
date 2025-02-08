# \LkeControlPlaneAclApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lke_cluster_acl**](LkeControlPlaneAclApi.md#delete_lke_cluster_acl) | **DELETE** /{apiVersion}/lke/clusters/{clusterId}/control_plane_acl | Delete the control plane access control list
[**get_lke_cluster_acl**](LkeControlPlaneAclApi.md#get_lke_cluster_acl) | **GET** /{apiVersion}/lke/clusters/{clusterId}/control_plane_acl | Get the control plane access control list
[**put_lke_cluster_acl**](LkeControlPlaneAclApi.md#put_lke_cluster_acl) | **PUT** /{apiVersion}/lke/clusters/{clusterId}/control_plane_acl | Update the control plane access control list



## delete_lke_cluster_acl

> serde_json::Value delete_lke_cluster_acl(api_version, cluster_id)
Delete the control plane access control list

Disable control plane access controls and deletes all rules. This has the same effect as calling `PUT` with an acl json map value of `{“enabled” : false}`. __Note__: control plane ACLs may not currently be available to all users.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke cluster-acl-delete 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

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


## get_lke_cluster_acl

> models::GetLkeClusterAcl200Response get_lke_cluster_acl(api_version, cluster_id)
Get the control plane access control list

Get a specific cluster's control plane access control List. __Note__: control plane ACLs may not currently be available to all users.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke cluster-acl-view 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **i32** | ID of the Kubernetes cluster to look up. | [required] |

### Return type

[**models::GetLkeClusterAcl200Response**](get_lke_cluster_acl_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lke_cluster_acl

> models::PutLkeClusterAcl200Response put_lke_cluster_acl(api_version, cluster_id, put_lke_cluster_acl_request)
Update the control plane access control list

Updates a specific cluster's control plane access control list. __Note__: control plane ACLs may not currently be available to all users.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke cluster-acl-update 12345 \\       --acl.enabled true \\       --acl.addresses.ipv4 \"203.0.113.1\" \\       --acl.addresses.ipv6 \"2001:db8:1234:abcd::/64\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **i32** | ID of the Kubernetes cluster to look up. | [required] |
**put_lke_cluster_acl_request** | Option<[**PutLkeClusterAclRequest**](PutLkeClusterAclRequest.md)> | The fields to update the cluster. |  |

### Return type

[**models::PutLkeClusterAcl200Response**](put_lke_cluster_acl_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

