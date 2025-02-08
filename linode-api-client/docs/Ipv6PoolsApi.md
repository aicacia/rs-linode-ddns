# \Ipv6PoolsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_ipv6_pools**](Ipv6PoolsApi.md#get_ipv6_pools) | **GET** /{apiVersion}/networking/ipv6/pools | List IPv6 pools



## get_ipv6_pools

> models::GetIpv6Pools200Response get_ipv6_pools(api_version, page, page_size)
List IPv6 pools

Displays the IPv6 pools on your Account. A pool of IPv6 addresses are routed to all of your Linodes in a single [region](https://techdocs.akamai.com/linode-api/reference/get-regions). Any Linode on your Account may bring up any address in this pool at any time, with no external configuration required.   <<LB>>  ---   - __CLI__.      ```     linode-cli networking v6-pools     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     ips:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetIpv6Pools200Response**](get_ipv6_pools_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

