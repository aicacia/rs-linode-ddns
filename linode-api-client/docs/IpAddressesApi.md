# \IpAddressesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_linode_ip**](IpAddressesApi.md#delete_linode_ip) | **DELETE** /{apiVersion}/linode/instances/{linodeId}/ips/{address} | Delete an IPv4 address
[**get_ip**](IpAddressesApi.md#get_ip) | **GET** /{apiVersion}/networking/ips/{address} | Get an IP address
[**get_ips**](IpAddressesApi.md#get_ips) | **GET** /{apiVersion}/networking/ips | List IP addresses
[**get_linode_ip**](IpAddressesApi.md#get_linode_ip) | **GET** /{apiVersion}/linode/instances/{linodeId}/ips/{address} | Get a Linode's IP address
[**get_linode_ips**](IpAddressesApi.md#get_linode_ips) | **GET** /{apiVersion}/linode/instances/{linodeId}/ips | Get networking information
[**get_vpc_ips**](IpAddressesApi.md#get_vpc_ips) | **GET** /{apiVersion}/vpcs/{vpcId}/ips | List a VPC's IP addresses
[**get_vpcs_ips**](IpAddressesApi.md#get_vpcs_ips) | **GET** /{apiVersion}/vpcs/ips | List VPC IP addresses
[**post_add_linode_ip**](IpAddressesApi.md#post_add_linode_ip) | **POST** /{apiVersion}/linode/instances/{linodeId}/ips | Allocate an IPv4 address
[**post_allocate_ip**](IpAddressesApi.md#post_allocate_ip) | **POST** /{apiVersion}/networking/ips | Allocate an IP address
[**post_assign_ips**](IpAddressesApi.md#post_assign_ips) | **POST** /{apiVersion}/networking/ips/assign | Assign IP addresses
[**post_share_ips**](IpAddressesApi.md#post_share_ips) | **POST** /{apiVersion}/networking/ips/share | Share IP addresses
[**put_ip**](IpAddressesApi.md#put_ip) | **PUT** /{apiVersion}/networking/ips/{address} | Update an IP address's RDNS
[**put_linode_ip**](IpAddressesApi.md#put_linode_ip) | **PUT** /{apiVersion}/linode/instances/{linodeId}/ips/{address} | Update an IP address's RDNS for a Linode



## delete_linode_ip

> serde_json::Value delete_linode_ip(api_version, linode_id, address)
Delete an IPv4 address

Deletes a public or private IPv4 address associated with this Linode. This will fail if it is the Linode's last remaining public IPv4 address, or if the address has a 1:1 NAT with an active VPC Subnet address.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes ip-delete 97.107.143.141     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The ID of the Linode. | [required] |
**address** | **String** | The IP address. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ip

> models::GetIp200Response get_ip(api_version, address)
Get an IP address

Returns information about a single IP Address on your Account.   <<LB>>  ---   - __CLI__.      ```     linode-cli networking ip-view 97.107.143.141     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     ips:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**address** | **String** | The address to operate on. | [required] |

### Return type

[**models::GetIp200Response**](get_ip_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ips

> models::GetIps200Response get_ips(api_version, skip_ipv6_rdns)
List IP addresses

Returns a paginated list of IP addresses on your account, excluding private addresses.  __Note__. Use the `skip_ipv6_rdns` query string to improve performance if your application frequently accesses this operation and doesn't require IPv6 RDNS data.   <<LB>>  ---   - __CLI__.      ```     linode-cli networking ips-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     ips:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**skip_ipv6_rdns** | Option<**bool**> | When `true`, the `rdns` property for any `ipv6` type addresses always returns `null` regardless of whether RDNS data exists for the address. |  |[default to false]

### Return type

[**models::GetIps200Response**](get_ips_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_ip

> models::GetLinodeIp200Response get_linode_ip(api_version, linode_id, address)
Get a Linode's IP address

View information about the specified IP address associated with the specified Linode.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes ip-view 123 97.107.143.141     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The ID of the Linode. | [required] |
**address** | **String** | The IP address. | [required] |

### Return type

[**models::GetLinodeIp200Response**](get_linode_ip_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_ips

> models::GetLinodeIps200Response get_linode_ips(api_version, linode_id)
Get networking information

Returns networking information for a single Linode.  __Note__. If the target Linode has several configuration profiles that include a Virtual Private Cloud (VPC) interface, address information for all of VPCs will be listed in the response.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes ips-list 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up. | [required] |

### Return type

[**models::GetLinodeIps200Response**](get_linode_ips_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vpc_ips

> models::GetVpcIps200Response get_vpc_ips(api_version, vpc_id, page, page_size)
List a VPC's IP addresses

Returns a paginated list of IP addresses for a single VPC.   <<LB>>  ---   - __CLI__.      ```     linode-cli vpcs ip-list 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     ips:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**vpc_id** | **i32** | The `id` of the VPC. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetVpcIps200Response**](get_vpc_ips_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vpcs_ips

> models::GetVpcsIps200Response get_vpcs_ips(api_version, page, page_size)
List VPC IP addresses

Returns a paginated list of all VPC IP addresses and address ranges on your account.  __Note__. If a Linode has several configuration profiles that include a VPC interface, address information for all of them is listed in the response. Since VPCs can use the same address space, you may see duplicate IP addresses.   <<LB>>  ---   - __CLI__.      ```     linode-cli vpcs ips-all-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     ips:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetVpcsIps200Response**](get_vpcs_ips_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_add_linode_ip

> models::PostAddLinodeIp200Response post_add_linode_ip(api_version, linode_id, post_add_linode_ip_request)
Allocate an IPv4 address

Allocates a public or private IPv4 address to a Linode. Public IP Addresses, after the one included with each Linode, incur an additional monthly charge. If you need an additional public IP Address you must request one - please [Open a support ticket](https://techdocs.akamai.com/linode-api/reference/post-ticket). You may not add more than one private IPv4 address to a single Linode.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes ip-add 123 \\   --type ipv4 \\   --public true     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**post_add_linode_ip_request** | [**PostAddLinodeIpRequest**](PostAddLinodeIpRequest.md) | Information about the address you are creating. | [required] |

### Return type

[**models::PostAddLinodeIp200Response**](post_add_linode_ip_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_allocate_ip

> models::PostAllocateIp200Response post_allocate_ip(api_version, post_allocate_ip_request)
Allocate an IP address

Allocates a new IPv4 Address on your Account. The Linode must be configured to support additional addresses - please [Open a support ticket](https://techdocs.akamai.com/linode-api/reference/post-ticket) requesting additional addresses before attempting allocation.   <<LB>>  ---   - __CLI__.      ```     linode-cli networking ip-add \\   --type ipv4 \\   --public true \\   --linode_id 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     ips:read_write linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_allocate_ip_request** | [**PostAllocateIpRequest**](PostAllocateIpRequest.md) | Information about the address you are creating. | [required] |

### Return type

[**models::PostAllocateIp200Response**](post_allocate_ip_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_assign_ips

> serde_json::Value post_assign_ips(api_version, post_assign_ips_request)
Assign IP addresses

Assign multiple IPv4 addresses and/or IPv6 ranges to multiple Linodes in one Region. This allows swapping, shuffling, or otherwise reorganizing IPs to your Linodes.  The following restrictions apply:  - All Linodes involved must have at least one public IPv4 address after assignment. - Linodes may have no more than one assigned private IPv4 address. - Linodes may have no more than one assigned IPv6 range. - Shared IP addresses cannot be swapped between Linodes.  [Open a support ticket](https://techdocs.akamai.com/linode-api/reference/post-ticket) to request additional IPv4 addresses or IPv6 ranges beyond standard account limits.  __Note__. Removing an IP address that has been set as a Managed Linode's `ssh.ip` causes the Managed Linode's SSH access settings to reset to their default values.  To view and configure Managed Linode SSH settings, use the following operations:  - [Get a Linode's managed settings](https://techdocs.akamai.com/linode-api/reference/get-managed-linode-setting) - [Update a Linode's managed settings](https://techdocs.akamai.com/linode-api/reference/put-managed-linode-setting)  __Note__. Addresses with an active 1:1 NAT to a VPC Interface address cannot be assigned to other Linodes.   <<LB>>  ---   - __CLI__.      ```     linode-cli networking ip-assign \\   --region us-east \\   --assignments.address 192.0.2.1 \\   --assignments.linode_id 123 \\   --assignments.address 2001:db8:3c4d:15::/64 \\   --assignments.linode_id 234     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     ips:read_write linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_assign_ips_request** | [**PostAssignIpsRequest**](PostAssignIpsRequest.md) | Information about what IPv4 address or IPv6 range to assign, and to which Linode. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_share_ips

> serde_json::Value post_share_ips(api_version, post_share_ips_request)
Share IP addresses

Configure shared IPs.  IP sharing allows IP address reassignment (also referred to as IP failover) from one Linode to another if the primary Linode becomes unresponsive. This means that requests to the primary Linode's IP address can be automatically rerouted to secondary Linodes at the configured shared IP addresses.  IP failover requires configuration of a [BGP based failover service](https://techdocs.akamai.com/cloud-computing/docs/configure-failover-on-a-compute-instance) within the internal system of the primary Linode.  __Note__. A public IPv4 address cannot be shared if it is configured for a 1:1 NAT on a `vpc` purpose Configuration Profile Interface.   <<LB>>  ---   - __CLI__.      ```     linode-cli networking ip-share \\   --linode_id 123 \\   --ips 192.0.2.1 \\   --ips 2001:db8:3c4d:15::     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     ips:read_write linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

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


## put_ip

> models::PutIp200Response put_ip(api_version, address, put_ip_request)
Update an IP address's RDNS

Sets RDNS on an IP Address. Forward DNS must already be set up for reverse DNS to be applied. If you set the RDNS to `null` for public IPv4 addresses, it will be reset to the default _ip.linodeusercontent.com_ RDNS value.   <<LB>>  ---   - __CLI__.      ```     linode-cli networking ip-update \\   203.0.113.1 \\   --rdns \"test.example.org\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     ips:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**address** | **String** | The address to operate on. | [required] |
**put_ip_request** | [**PutIpRequest**](PutIpRequest.md) | The information to update. | [required] |

### Return type

[**models::PutIp200Response**](put_ip_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_linode_ip

> models::PutLinodeIp200Response put_linode_ip(api_version, linode_id, address, put_linode_ip_request)
Update an IP address's RDNS for a Linode

Updates the reverse DNS (RDNS) for a Linode's IP Address. This may be done for both IPv4 and IPv6 addresses.  Setting the RDNS to `null` for a public IPv4 address, resets it to the default `ip.linodeusercontent.com` RDNS value.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes ip-update 123 \\   203.0.113.1 \\   --rdns test.example.org     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The ID of the Linode. | [required] |
**address** | **String** | The IP address. | [required] |
**put_linode_ip_request** | Option<[**PutLinodeIpRequest**](PutLinodeIpRequest.md)> | The information to update for the IP address. |  |

### Return type

[**models::PutLinodeIp200Response**](put_linode_ip_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

