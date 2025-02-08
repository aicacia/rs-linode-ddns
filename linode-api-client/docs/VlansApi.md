# \VlansApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_vlan**](VlansApi.md#delete_vlan) | **DELETE** /{apiVersion}/networking/vlans/{regionId}/{label} | Delete a VLAN
[**get_vlans**](VlansApi.md#get_vlans) | **GET** /{apiVersion}/networking/vlans | List VLANs



## delete_vlan

> serde_json::Value delete_vlan(api_version, label, region_id)
Delete a VLAN

This operation deletes a VLAN. You can't delete a VLAN if it's still attached to a Linode. There are a few ways to detach it: - [Update](https://techdocs.akamai.com/linode-api/reference/put-linode-config) the active configuration profile to remove the VLAN interface, then [reboot](https://techdocs.akamai.com/linode-api/reference/post-reboot-linode-instance) the Linode. - [Create a config profile](https://techdocs.akamai.com/linode-api/reference/post-add-linode-config) without the VLAN interface, then [reboot](https://techdocs.akamai.com/linode-api/reference/post-reboot-linode-instance) the Linode into the new configuration profile. - [Delete](https://techdocs.akamai.com/linode-api/reference/delete-linode-instance) the Linode.  To run this operation, you need `read_write` grants to Linodes that use the VLAN.  A successful request triggers a `vlan_delete` event.  > 📘 > > VLANs without any attached Linodes are periodically cleaned up by the system.   <<LB>>  ---   - __CLI__.      ```     linode-cli vlans delete $regionId $label     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**label** | **String** | The label of the VLAN to be deleted. | [required] |
**region_id** | **String** | The VLAN's region. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vlans

> models::GetVlans200Response get_vlans(api_version, page, page_size)
List VLANs

Returns a list of all Virtual Local Area Networks (VLANs) on your Account. VLANs provide a mechanism for secure communication between two or more Linodes that are assigned to the same VLAN and are both within the same Layer 2 broadcast domain.  VLANs are created and attached to Linodes by using the `interfaces` property for the following operations:  - [Create a Linode](https://techdocs.akamai.com/linode-api/reference/post-linode-instance) - [Create a config profile](https://techdocs.akamai.com/linode-api/reference/post-add-linode-config) - [Update a config profile](https://techdocs.akamai.com/linode-api/reference/put-linode-config)  There are several ways to detach a VLAN from a Linode:  - [Update](https://techdocs.akamai.com/linode-api/reference/put-linode-config) the active Configuration Profile to remove the VLAN Interface, then [reboot](https://techdocs.akamai.com/linode-api/reference/post-reboot-linode-instance) the Linode. - [Create a config profile](https://techdocs.akamai.com/linode-api/reference/post-add-linode-config) without the VLAN Interface, then [reboot](https://techdocs.akamai.com/linode-api/reference/post-reboot-linode-instance) the Linode into the new Configuration Profile. - [Delete](https://techdocs.akamai.com/linode-api/reference/delete-linode-instance) the Linode.  __Note__. Only Next Generation Network (NGN) data centers support VLANs. Run the [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions) operation to view the capabilities of data center regions. If a VLAN is attached to your Linode and you attempt to migrate or clone it to a non-NGN data center, the migration or cloning will not initiate. If a Linode cannot be migrated because of an incompatibility, you will be prompted to select a different data center or contact support.  __Note__. See the [VLANs Overview](https://www.linode.com/docs/products/networking/vlans/#technical-specifications) to view additional specifications and limitations.   <<LB>>  ---   - __CLI__.      ```     linode-cli vlans list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetVlans200Response**](get_vlans_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

