# \StatisticsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_linode_stats**](StatisticsApi.md#get_linode_stats) | **GET** /{apiVersion}/linode/instances/{linodeId}/stats | Get Linode statistics
[**get_linode_stats_by_year_month**](StatisticsApi.md#get_linode_stats_by_year_month) | **GET** /{apiVersion}/linode/instances/{linodeId}/stats/{year}/{month} | Get monthly statistics
[**get_managed_stats**](StatisticsApi.md#get_managed_stats) | **GET** /{apiVersion}/managed/stats | List managed stats
[**get_node_balancer_stats**](StatisticsApi.md#get_node_balancer_stats) | **GET** /{apiVersion}/nodebalancers/{nodeBalancerId}/stats | Get NodeBalancer statistics



## get_linode_stats

> models::GetLinodeStats200Response get_linode_stats(api_version, linode_id)
Get Linode statistics

Returns CPU, IO, IPv4, and IPv6 statistics for your Linode for the past 24 hours.   <<LB>>  ---   - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up. | [required] |

### Return type

[**models::GetLinodeStats200Response**](get_linode_stats_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_stats_by_year_month

> models::GetLinodeStats200Response get_linode_stats_by_year_month(api_version, linode_id, year, month)
Get monthly statistics

Returns statistics for a specific month. The year/month values must be either a date in the past, or the current month. If the current month, statistics will be retrieved for the past 30 days.   <<LB>>  ---   - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**year** | **i32** | Numeric value representing the year to look up. | [required] |
**month** | **i32** | Numeric value representing the month to look up. | [required] |

### Return type

[**models::GetLinodeStats200Response**](get_linode_stats_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_managed_stats

> models::GetManagedStats200Response get_managed_stats(api_version)
List managed stats

Returns a list of Managed Stats on your Account in the form of x and y data points. You can use these data points to plot your own graph visualizations. These stats reflect the last 24 hours of combined usage across all managed Linodes on your account giving you a high-level snapshot of data for the following:  - cpu - disk - swap - network in - network out  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed stats-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetManagedStats200Response**](get_managed_stats_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_node_balancer_stats

> models::GetNodeBalancerStats200Response get_node_balancer_stats(api_version, node_balancer_id)
Get NodeBalancer statistics

Returns detailed statistics about the requested NodeBalancer.   <<LB>>  ---   - __OAuth scopes__.      ```     nodebalancers:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**node_balancer_id** | **i32** | The ID of the NodeBalancer to access. | [required] |

### Return type

[**models::GetNodeBalancerStats200Response**](get_node_balancer_stats_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

