# \ClusterDashboardApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lke_cluster_dashboard**](ClusterDashboardApi.md#get_lke_cluster_dashboard) | **GET** /{apiVersion}/lke/clusters/{clusterId}/dashboard | Get a Kubernetes cluster dashboard URL



## get_lke_cluster_dashboard

> models::GetLkeClusterDashboard200Response get_lke_cluster_dashboard(api_version, cluster_id)
Get a Kubernetes cluster dashboard URL

Get a [Kubernetes Dashboard](https://github.com/kubernetes/dashboard) access URL for this Cluster, which enables performance of administrative tasks through a web interface.  Dashboards are installed for Clusters by default.  To access the Cluster Dashboard login prompt, enter the URL in a web browser. Select either __Token__ or __Kubeconfig__ authentication, then select __Sign in__.  For additional guidance on using the Cluster Dashboard, see the [Navigating the Cluster Dashboard](https://www.linode.com/docs/guides/using-the-kubernetes-dashboard-on-lke/#navigating-the-cluster-dashboard) section of our guide on [Using the Kubernetes Dashboard on LKE](https://www.linode.com/docs/guides/using-the-kubernetes-dashboard-on-lke/).   <<LB>>  ---   - __CLI__.      ```     linode-cli lke cluster-dashboard-url 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**cluster_id** | **i32** | ID of the Kubernetes cluster to look up. | [required] |

### Return type

[**models::GetLkeClusterDashboard200Response**](get_lke_cluster_dashboard_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

