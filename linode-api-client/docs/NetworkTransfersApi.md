# \NetworkTransfersApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_linode_transfer**](NetworkTransfersApi.md#get_linode_transfer) | **GET** /{apiVersion}/linode/instances/{linodeId}/transfer | Get a network transfer



## get_linode_transfer

> models::GetLinodeTransfer200Response get_linode_transfer(api_version, linode_id)
Get a network transfer

Returns a Linode's network transfer pool statistics for the current month.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes transfer-view 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up. | [required] |

### Return type

[**models::GetLinodeTransfer200Response**](get_linode_transfer_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

