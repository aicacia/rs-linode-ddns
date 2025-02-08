# \ClustersApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lke_cluster**](ClustersApi.md#delete_lke_cluster) | **DELETE** /{apiVersion}/lke/clusters/{clusterId} | Delete a Kubernetes cluster
[**get_lke_cluster**](ClustersApi.md#get_lke_cluster) | **GET** /{apiVersion}/lke/clusters/{clusterId} | Get a Kubernetes cluster
[**get_lke_clusters**](ClustersApi.md#get_lke_clusters) | **GET** /{apiVersion}/lke/clusters | List Kubernetes clusters
[**get_object_storage_cluster**](ClustersApi.md#get_object_storage_cluster) | **GET** /{apiVersion}/object-storage/clusters/{clusterId} | Get a cluster
[**get_object_storage_clusters**](ClustersApi.md#get_object_storage_clusters) | **GET** /{apiVersion}/object-storage/clusters | List clusters
[**post_lke_cluster**](ClustersApi.md#post_lke_cluster) | **POST** /{apiVersion}/lke/clusters | Create a Kubernetes cluster
[**post_lke_cluster_recycle**](ClustersApi.md#post_lke_cluster_recycle) | **POST** /{apiVersion}/lke/clusters/{clusterId}/recycle | Recycle cluster nodes
[**post_lke_cluster_regenerate**](ClustersApi.md#post_lke_cluster_regenerate) | **POST** /{apiVersion}/lke/clusters/{clusterId}/regenerate | Regenerate a Kubernetes cluster
[**put_lke_cluster**](ClustersApi.md#put_lke_cluster) | **PUT** /{apiVersion}/lke/clusters/{clusterId} | Update a Kubernetes cluster



## delete_lke_cluster

> serde_json::Value delete_lke_cluster(api_version, cluster_id)
Delete a Kubernetes cluster

Deletes a Cluster you have permission to `read_write`.  __Deleting a Cluster is a destructive action and cannot be undone.__  Deleting a Cluster:  - Deletes all Linodes in all pools within this Kubernetes cluster - Deletes all supporting Kubernetes services for this Kubernetes cluster (API server, etcd, etc) - Deletes all NodeBalancers created by this Kubernetes cluster - Does not delete any of the volumes created by this Kubernetes cluster   <<LB>>  ---   - __CLI__.      ```     linode-cli lke cluster-delete 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

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


## get_lke_cluster

> models::GetLkeClusters200ResponseDataInner get_lke_cluster(api_version, cluster_id)
Get a Kubernetes cluster

Get a specific Cluster by ID.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke cluster-view 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **i32** | ID of the Kubernetes cluster to look up. | [required] |

### Return type

[**models::GetLkeClusters200ResponseDataInner**](get_lke_clusters_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lke_clusters

> models::GetLkeClusters200Response get_lke_clusters(api_version)
List Kubernetes clusters

Lists current Kubernetes clusters available on your account.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke clusters-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetLkeClusters200Response**](get_lke_clusters_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_storage_cluster

> models::GetObjectStorageClusters200ResponseDataInner get_object_storage_cluster(api_version, cluster_id)
Get a cluster

__Deprecated__ Returns a single Object Storage cluster.  > 📘 > > This displays deprecated `clusterId` values that represent regions used with older versions of the API. It's maintained for backward compatibility. Run [Get a region](https://techdocs.akamai.com/linode-api/reference/get-region), instead.   <<LB>>  ---   - __CLI__.      ```     linode-cli object-storage clusters-view us-east-1     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **String** | Identifies a cluster where this bucket lives. For backward compatibility with Object Storage in this API.  > 📘 > > You can use the applicable `regionId`, for example `us-west`, in place of the `clusterId`, for example, `us-west-1`. Run [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions) to see all regions. | [required] |

### Return type

[**models::GetObjectStorageClusters200ResponseDataInner**](get_object_storage_clusters_200_response_data_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_storage_clusters

> models::GetObjectStorageClusters200Response get_object_storage_clusters(api_version)
List clusters

__Deprecated__ Returns a paginated list of available Object Storage legacy clusters.  > 📘 > > This displays deprecated `clusterId` values that represent regions used with older versions of the API. It's maintained for backward compatibility. Run [Get a region](https://techdocs.akamai.com/linode-api/reference/get-region), instead.   <<LB>>  ---   - __CLI__.      ```     linode-cli object-storage clusters-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetObjectStorageClusters200Response**](get_object_storage_clusters_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lke_cluster

> models::GetLkeClusters200ResponseDataInner post_lke_cluster(api_version, post_lke_cluster_request)
Create a Kubernetes cluster

Creates a Kubernetes cluster. The Kubernetes cluster will be created asynchronously. You can use the events system to determine when the Kubernetes cluster is ready to use. Please note that it often takes 2-5 minutes before the [Kubernetes API endpoints](https://techdocs.akamai.com/linode-api/reference/get-lke-cluster-api-endpoints) and the [Kubeconfig file](https://techdocs.akamai.com/linode-api/reference/get-lke-cluster-kubeconfig) for the new cluster are ready.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke cluster-create \\   --label cluster12345 \\   --region us-central \\   --k8s_version 1.31 \\   --control_plane.high_availability true \\   --node_pools.type g6-standard-4 --node_pools.count 6 \\   --node_pools.type g6-standard-8 --node_pools.count 3 \\   --node_pools.autoscaler.enabled true \\   --node_pools.autoscaler.max 12 \\   --node_pools.autoscaler.min 3 \\   --tags ecomm     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_lke_cluster_request** | Option<[**PostLkeClusterRequest**](PostLkeClusterRequest.md)> | Configuration for the Kubernetes cluster. |  |

### Return type

[**models::GetLkeClusters200ResponseDataInner**](get_lke_clusters_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lke_cluster_recycle

> serde_json::Value post_lke_cluster_recycle(api_version, cluster_id)
Recycle cluster nodes

Recycles all nodes in all pools of a designated Kubernetes Cluster. All Linodes within the Cluster will be deleted and replaced with new Linodes on a rolling basis, which may take several minutes. Replacement Nodes are installed with the latest available patch version for the Cluster's current Kubernetes minor release.  __Any local storage on deleted Linodes (such as `hostPath` and `emptyDir` volumes, or `local` PersistentVolumes) will be erased.__   <<LB>>  ---   - __CLI__.      ```     linode-cli lke cluster-nodes-recycle 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **i32** | ID of the Kubernetes cluster which contains nodes to be recycled. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lke_cluster_regenerate

> serde_json::Value post_lke_cluster_regenerate(api_version, cluster_id, post_lke_cluster_regenerate_request)
Regenerate a Kubernetes cluster

Regenerate the Kubeconfig file and/or the service account token for a Cluster.  This is a helper operation that allows performing both the [Delete a Kubeconfig](https://techdocs.akamai.com/linode-api/reference/delete-lke-cluster-kubeconfig) and the [Delete a service token](https://techdocs.akamai.com/linode-api/reference/delete-lke-service-token) operations with a single request.  When using this operation, at least one of `kubeconfig` or `servicetoken` is required.  __Note__. When regenerating a service account token, the Cluster's control plane components and Linode CSI drivers are also restarted and configured with the new token. High Availability Clusters should not experience any disruption, while standard Clusters may experience brief control plane downtime while components are restarted.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke regenerate 12345 \\     --kubeconfig true \\     --servicetoken true     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **i32** | ID of the target Kubernetes cluster. | [required] |
**post_lke_cluster_regenerate_request** | Option<[**PostLkeClusterRegenerateRequest**](PostLkeClusterRegenerateRequest.md)> | The Kubernetes Cluster Regenerate request object. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lke_cluster

> models::PutLkeCluster200Response put_lke_cluster(api_version, cluster_id, put_lke_cluster_request)
Update a Kubernetes cluster

Updates a Kubernetes cluster.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke cluster-update 12345 \\   --label lkecluster54321 \\   --control_plane.high_availability true \\   --k8s_version 1.31 \\   --tags ecomm \\   --tags blog \\   --tags prod \\   --tags monitoring     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **i32** | ID of the Kubernetes cluster to look up. | [required] |
**put_lke_cluster_request** | Option<[**PutLkeClusterRequest**](PutLkeClusterRequest.md)> | The fields to update the Kubernetes cluster. |  |

### Return type

[**models::PutLkeCluster200Response**](put_lke_cluster_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

