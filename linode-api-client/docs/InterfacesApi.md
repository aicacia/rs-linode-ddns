# \InterfacesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_linode_config_interface**](InterfacesApi.md#delete_linode_config_interface) | **DELETE** /{apiVersion}/linode/instances/{linodeId}/configs/{configId}/interfaces/{interfaceId} | Delete a configuration profile interface
[**get_linode_config_interface**](InterfacesApi.md#get_linode_config_interface) | **GET** /{apiVersion}/linode/instances/{linodeId}/configs/{configId}/interfaces/{interfaceId} | Get a configuration profile interface
[**get_linode_config_interfaces**](InterfacesApi.md#get_linode_config_interfaces) | **GET** /{apiVersion}/linode/instances/{linodeId}/configs/{configId}/interfaces | List configuration profile interfaces
[**post_linode_config_interface**](InterfacesApi.md#post_linode_config_interface) | **POST** /{apiVersion}/linode/instances/{linodeId}/configs/{configId}/interfaces | Add a configuration profile interface
[**post_linode_config_interfaces**](InterfacesApi.md#post_linode_config_interfaces) | **POST** /{apiVersion}/linode/instances/{linodeId}/configs/{configId}/interfaces/order | Reorder configuration profile interfaces
[**put_linode_config_interface**](InterfacesApi.md#put_linode_config_interface) | **PUT** /{apiVersion}/linode/instances/{linodeId}/configs/{configId}/interfaces/{interfaceId} | Update a configuration profile interface



## delete_linode_config_interface

> serde_json::Value delete_linode_config_interface(api_version, linode_id, config_id, interface_id)
Delete a configuration profile interface

Deletes a configuration profile interface from a specific configuration profile, on a specific Linode.  - To access this operation, your user needs the `read_write` [grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) for the Linode.  - A successful request triggers a `linode_config_update` [event](https://techdocs.akamai.com/linode-api/reference/get-events).  - You can't delete an active configuration profile interface. First, you need to shut down the associated Linode or restart it using another configuration profile.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes config-interface-delete $linodeId $configId $interfaceId     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The `id` of the Linode. | [required] |
**config_id** | **i32** | The `id` of the Configuration Profile. | [required] |
**interface_id** | **i32** | The `id` of the Linode Configuration Profile Interface. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_config_interface

> models::GetLinodeConfigInterface200Response get_linode_config_interface(api_version, linode_id, config_id, interface_id)
Get a configuration profile interface

Returns a single configuration profile interface. To access this operation, your user needs at least the `read_only` [grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) for the Linode.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes config-interface-view $linodeId $configId $interfaceId     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The `id` of the Linode. | [required] |
**config_id** | **i32** | The `id` of the Configuration Profile. | [required] |
**interface_id** | **i32** | The `id` of the Linode Configuration Profile Interface. | [required] |

### Return type

[**models::GetLinodeConfigInterface200Response**](get_linode_config_interface_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_config_interfaces

> Vec<models::GetLinodeConfigInterfaces200ResponseInner> get_linode_config_interfaces(api_version, linode_id, config_id)
List configuration profile interfaces

Returns all configuration profile interfaces assigned to a specific configuration profile, on a specific Linode. To access this operation, your user needs the `read_write` [grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) for the Linode.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes config-interfaces-list $linodeId $configId     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The `id` of the Linode. | [required] |
**config_id** | **i32** | The `id` of the Configuration Profile. | [required] |

### Return type

[**Vec<models::GetLinodeConfigInterfaces200ResponseInner>**](get_linode_config_interfaces_200_response_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_linode_config_interface

> models::PostLinodeConfigInterface200Response post_linode_config_interface(api_version, linode_id, config_id, post_linode_config_interface_request)
Add a configuration profile interface

Creates and appends a single interface to the end of the `interfaces` array for an existing configuration profile. After you add the interface, you need to reboot the target Linode.  - To access this operation, your user needs the `read_write` [grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) for the Linode.  - A successful request triggers a `linode_config_update` [event](https://techdocs.akamai.com/linode-api/reference/get-events).  - Only one interface can be set as `primary`. Setting `primary` to `true` for an interface sets all other interfaces to `false`.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes config-interface-add $linodeId $configId \\   --purpose vpc \\   --primary false \\   --subnet_id 101 \\   --ipv4.vpc \"10.0.1.2\" \\   --ipv4.nat_1_1 \"203.0.113.2\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The `id` of the Linode. | [required] |
**config_id** | **i32** | The `id` of the Configuration Profile. | [required] |
**post_linode_config_interface_request** | [**PostLinodeConfigInterfaceRequest**](PostLinodeConfigInterfaceRequest.md) |  | [required] |

### Return type

[**models::PostLinodeConfigInterface200Response**](post_linode_config_interface_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_linode_config_interfaces

> serde_json::Value post_linode_config_interfaces(api_version, linode_id, config_id, post_linode_config_interfaces_request)
Reorder configuration profile interfaces

Reorders the existing Interfaces of a Configuration Profile.  - The User accessing this operation must have `read_write` grants to the Linode.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes config-interfaces-order $linodeId $configId \\   --ids 101 --ids 102 --ids 103     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The `id` of the Linode. | [required] |
**config_id** | **i32** | The `id` of the Configuration Profile. | [required] |
**post_linode_config_interfaces_request** | [**PostLinodeConfigInterfacesRequest**](PostLinodeConfigInterfacesRequest.md) | The desired Interface order for the Configuration Profile. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_linode_config_interface

> models::PutLinodeConfigInterface200Response put_linode_config_interface(api_version, linode_id, config_id, interface_id, put_linode_config_interface_request)
Update a configuration profile interface

Update a `vpc` or `public` configuration profile interface for a specific configuration profile, on a specific Linode.  - To access this operation, your user needs the `read_write` [grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) for the Linode.  - A successful request triggers a `linode_config_update` [event](https://techdocs.akamai.com/linode-api/reference/get-events).  - Only certain attributes can be updated for a configuration profile interface. You need to [add](https://techdocs.akamai.com/linode-api/reference/post-linode-config-interface) a new configuration profile interface on your Linode if you need new values for any other attribute. Here are the supported objects, based on the interface's `purpose`:    - `public`. The `primary` attribute.    - `vpc`. The `ip_ranges`, `ipv4`, or `primary` attributes.  - You can't update a configuration profile with a `purpose` of `vlan`.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes config-interface-update $linodeId $configId $interfaceId \\   --primary true \\   --ipv4.vpc \"10.0.1.2\" \\   --ipv4.nat_1_1 \"203.0.113.2\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The `id` of the Linode. | [required] |
**config_id** | **i32** | The `id` of the Configuration Profile. | [required] |
**interface_id** | **i32** | The `id` of the Linode Configuration Profile Interface. | [required] |
**put_linode_config_interface_request** | [**PutLinodeConfigInterfaceRequest**](PutLinodeConfigInterfaceRequest.md) |  | [required] |

### Return type

[**models::PutLinodeConfigInterface200Response**](put_linode_config_interface_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

