# \Ipv6RangesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_ipv6_range**](Ipv6RangesApi.md#delete_ipv6_range) | **DELETE** /{apiVersion}/networking/ipv6/ranges/{range} | Delete an IPv6 range
[**get_ipv6_range**](Ipv6RangesApi.md#get_ipv6_range) | **GET** /{apiVersion}/networking/ipv6/ranges/{range} | Get an IPv6 range
[**get_ipv6_ranges**](Ipv6RangesApi.md#get_ipv6_ranges) | **GET** /{apiVersion}/networking/ipv6/ranges | List IPv6 ranges
[**post_ipv6_range**](Ipv6RangesApi.md#post_ipv6_range) | **POST** /{apiVersion}/networking/ipv6/ranges | Create an IPv6 range



## delete_ipv6_range

> serde_json::Value delete_ipv6_range(api_version, range)
Delete an IPv6 range

Removes this IPv6 range from your account and disconnects the range from any assigned Linodes.  __Note__. Shared IPv6 ranges cannot be deleted at this time. Please contact Customer Support for assistance.   <<LB>>  ---   - __CLI__.      ```     linode-cli networking v6-range-delete 2001:0db8::     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     ips:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**range** | **String** | The IPv6 range to access. Corresponds to the `range` property of objects returned from the [List IPv6 ranges](https://techdocs.akamai.com/linode-api/reference/get-ipv6-ranges) operation.  __Note__. Omit the prefix length of the IPv6 range. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ipv6_range

> models::GetIpv6Range200Response get_ipv6_range(api_version, range)
Get an IPv6 range

View IPv6 range information.   <<LB>>  ---   - __CLI__.      ```     linode-cli networking v6-range-view 2001:0db8::     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     ips:read     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**range** | **String** | The IPv6 range to access. Corresponds to the `range` property of objects returned from the [List IPv6 ranges](https://techdocs.akamai.com/linode-api/reference/get-ipv6-ranges) operation.  __Note__. Omit the prefix length of the IPv6 range. | [required] |

### Return type

[**models::GetIpv6Range200Response**](get_ipv6_range_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ipv6_ranges

> models::GetIpv6Ranges200Response get_ipv6_ranges(api_version, page, page_size)
List IPv6 ranges

Displays the IPv6 ranges on your Account.    - An IPv6 range is a `/64` or `/56` block of IPv6 addresses routed to a single Linode in a given [region](https://techdocs.akamai.com/linode-api/reference/get-regions).    - Your Linode is responsible for routing individual addresses in the range, or handling traffic for all the addresses in the range.    - Run the [Create an IPv6 range](https://techdocs.akamai.com/linode-api/reference/post-ipv6-range) operation to add a `/64` or `/56` block of IPv6 addresses to your account.   <<LB>>  ---   - __CLI__.      ```     linode-cli networking v6-ranges     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     ips:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetIpv6Ranges200Response**](get_ipv6_ranges_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_ipv6_range

> models::PostIpv6Range200Response post_ipv6_range(api_version, post_ipv6_range_request)
Create an IPv6 range

Creates an IPv6 Range and assigns it based on the provided Linode or route target IPv6 SLAAC address. See the `ipv6` property when running the [Get a Linode](https://techdocs.akamai.com/linode-api/reference/get-linode-instance) operation to view a Linode's IPv6 SLAAC address.  - Either `linode_id` or `route_target` is required in a request. - `linode_id` and `route_target` are mutually exclusive. Submitting values for both properties in a request results in an error. - Upon a successful request, an IPv6 range is created in the [region](https://techdocs.akamai.com/linode-api/reference/get-regions) that corresponds to the provided `linode_id` or `route_target`. - Your Linode is responsible for routing individual addresses in the range, or handling traffic for all the addresses in the range. - Run the [Assign IP addresses](https://techdocs.akamai.com/linode-api/reference/post-assign-ips) operation to re-assign IPv6 Ranges to your Linodes.  __Note__. The following restrictions apply:    - A Linode can only have one IPv6 range targeting its SLAAC address.   - An account can only have one IPv6 range in each [region](https://techdocs.akamai.com/linode-api/reference/get-regions).   - [Open a support ticket](https://techdocs.akamai.com/linode-api/reference/post-ticket) to request expansion of these restrictions.   <<LB>>  ---   - __CLI__.      ```     linode-cli networking v6-range-create \\   --linode_id 123 \\   --prefix_length 64     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     ips:read_write linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_ipv6_range_request** | [**PostIpv6RangeRequest**](PostIpv6RangeRequest.md) | Information about the IPv6 range to create. | [required] |

### Return type

[**models::PostIpv6Range200Response**](post_ipv6_range_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

