# \DevicesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_firewall_device**](DevicesApi.md#delete_firewall_device) | **DELETE** /{apiVersion}/networking/firewalls/{firewallId}/devices/{deviceId} | Delete a firewall device
[**delete_trusted_device**](DevicesApi.md#delete_trusted_device) | **DELETE** /{apiVersion}/profile/devices/{deviceId} | Revoke a trusted device
[**get_devices**](DevicesApi.md#get_devices) | **GET** /{apiVersion}/profile/devices | List trusted devices
[**get_firewall_device**](DevicesApi.md#get_firewall_device) | **GET** /{apiVersion}/networking/firewalls/{firewallId}/devices/{deviceId} | Get a firewall device
[**get_firewall_devices**](DevicesApi.md#get_firewall_devices) | **GET** /{apiVersion}/networking/firewalls/{firewallId}/devices | List firewall devices
[**get_trusted_device**](DevicesApi.md#get_trusted_device) | **GET** /{apiVersion}/profile/devices/{deviceId} | Get a trusted device
[**post_firewall_device**](DevicesApi.md#post_firewall_device) | **POST** /{apiVersion}/networking/firewalls/{firewallId}/devices | Create a firewall device



## delete_firewall_device

> serde_json::Value delete_firewall_device(api_version, firewall_id, device_id)
Delete a firewall device

Removes a Firewall Device, which removes a Firewall from the service it was assigned to by the Device. This removes all of the Firewall's Rules from the service. If any other Firewalls have been assigned to the service, then those Rules remain in effect.  - Assigned Linodes must not have any ongoing live migrations.  - A `firewall_device_remove` Event is generated when the Firewall Device is removed successfully.   <<LB>>  ---   - __CLI__.      ```     linode-cli firewalls device-delete 123 456     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     firewall:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**firewall_id** | **i32** | ID of the Firewall to access. | [required] |
**device_id** | **i32** | ID of the Firewall Device to access. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_trusted_device

> serde_json::Value delete_trusted_device(api_version, device_id)
Revoke a trusted device

Revoke an active TrustedDevice for your User.  Once a TrustedDevice is revoked, this device will have to log in again before accessing your Linode account.   <<LB>>  ---   - __CLI__.      ```     linode-cli profile device-revoke 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**device_id** | **i32** | The ID of the TrustedDevice. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_devices

> models::GetDevices200Response get_devices(api_version)
List trusted devices

Returns a paginated list of active TrustedDevices for your User. Browsers with an active Remember Me Session are logged into your account until the session expires or is revoked.   <<LB>>  ---   - __CLI__.      ```     linode-cli profile devices-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetDevices200Response**](get_devices_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_firewall_device

> models::GetFirewallDevices200ResponseDataInner get_firewall_device(api_version, firewall_id, device_id)
Get a firewall device

Returns information for a Firewall Device, which assigns a Firewall to a service (referred to as the Device's `entity`).   <<LB>>  ---   - __CLI__.      ```     linode-cli firewalls device-view \\   123 456     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     firewall:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**firewall_id** | **i32** | ID of the Firewall to access. | [required] |
**device_id** | **i32** | ID of the Firewall Device to access. | [required] |

### Return type

[**models::GetFirewallDevices200ResponseDataInner**](get_firewall_devices_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_firewall_devices

> models::GetFirewallDevices200Response get_firewall_devices(api_version, firewall_id, page, page_size)
List firewall devices

Returns a paginated list of a Firewall's Devices. A Firewall Device assigns a Firewall to a service (referred to as the Device's `entity`).   <<LB>>  ---   - __CLI__.      ```     linode-cli firewalls devices-list 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     firewall:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**firewall_id** | **i32** | ID of the Firewall to access. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetFirewallDevices200Response**](get_firewall_devices_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trusted_device

> models::GetTrustedDevice200Response get_trusted_device(api_version, device_id)
Get a trusted device

Returns a single active TrustedDevice for your User.   <<LB>>  ---   - __CLI__.      ```     linode-cli profile device-view 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**device_id** | **i32** | The ID of the TrustedDevice. | [required] |

### Return type

[**models::GetTrustedDevice200Response**](get_trusted_device_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_firewall_device

> models::GetFirewallDevices200ResponseDataInner post_firewall_device(api_version, firewall_id, post_firewall_device_request)
Create a firewall device

Creates a Firewall Device, which assigns a Firewall to a service (referred to as the Device's `entity`) and applies the Firewall's Rules to the device.  - Currently, Devices with `linode` and `nodebalancer` entity types are accepted.  - Firewalls only apply to inbound TCP traffic to NodeBalancers.  - A Firewall can be assigned to multiple services at a time.  - A service can have one assigned Firewall at a time.  - Assigned Linodes must not have any ongoing live migrations.  - A `firewall_device_add` Event is generated when the Firewall Device is added successfully.   <<LB>>  ---   - __CLI__.      ```     linode-cli firewalls device-create 123 \\   --id 456 \\   --type \"linode\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     firewall:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**firewall_id** | **i32** | ID of the Firewall to access. | [required] |
**post_firewall_device_request** | Option<[**PostFirewallDeviceRequest**](PostFirewallDeviceRequest.md)> |  |  |

### Return type

[**models::GetFirewallDevices200ResponseDataInner**](get_firewall_devices_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

