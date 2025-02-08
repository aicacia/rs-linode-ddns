# \Ipv4AddressesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_assign_ipv4s**](Ipv4AddressesApi.md#post_assign_ipv4s) | **POST** /{apiVersion}/networking/ipv4/assign | Assign IPv4s to Linodes
[**post_share_ipv4s**](Ipv4AddressesApi.md#post_share_ipv4s) | **POST** /{apiVersion}/networking/ipv4/share | Configure IPv4 sharing



## post_assign_ipv4s

> serde_json::Value post_assign_ipv4s(api_version, post_assign_ips_request)
Assign IPv4s to Linodes

This operation is equivalent to [Assign IP addresses](https://techdocs.akamai.com/linode-api/reference/post-assign-ips).  Assign multiple IPv4 addresses and/or IPv6 ranges to multiple Linodes in one Region. This allows swapping, shuffling, or otherwise reorganizing IPs to your Linodes.  The following restrictions apply:  - All Linodes involved must have at least one public IPv4 address after assignment. - Linodes may have no more than one assigned private IPv4 address. - Linodes may have no more than one assigned IPv6 range.  [Open a support ticket](https://techdocs.akamai.com/linode-api/reference/post-ticket) to request additional IPv4 addresses or IPv6 ranges beyond standard account limits.  __Note__. Removing an IP address that has been set as a Managed Linode's `ssh.ip` causes the Managed Linode's SSH access settings to reset to their default values.  To view and configure Managed Linode SSH settings, use the following operations: - [Get a Linode's managed settings](https://techdocs.akamai.com/linode-api/reference/get-managed-linode-setting) - [Update a Linode's managed settings](https://techdocs.akamai.com/linode-api/reference/put-managed-linode-setting)   <<LB>>  ---   - __OAuth scopes__.      ```     ips:read_write linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_assign_ips_request** | [**PostAssignIpsRequest**](PostAssignIpsRequest.md) | Information about what IPv4 address to assign, and to which Linode. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_share_ipv4s

> serde_json::Value post_share_ipv4s(api_version, post_share_ips_request)
Configure IPv4 sharing

This operation is equivalent to [Share IP addresses](https://techdocs.akamai.com/linode-api/reference/post-share-ips).  Configure shared IPs.  IP sharing allows IP address reassignment (also referred to as IP failover) from one Linode to another if the primary Linode becomes unresponsive. This means that requests to the primary Linode's IP address can be automatically rerouted to secondary Linodes at the configured shared IP addresses.  IP failover requires configuration of a [BGP based failover service](https://techdocs.akamai.com/cloud-computing/docs/configure-failover-on-a-compute-instance) within the internal system of the primary Linode.   <<LB>>  ---   - __OAuth scopes__.      ```     ips:read_write linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_share_ips_request** | [**PostShareIpsRequest**](PostShareIpsRequest.md) | Information about what IPs to share with which Linode. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

