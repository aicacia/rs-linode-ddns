# \ObjectStorageApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_object_storage_transfer**](ObjectStorageApi.md#get_object_storage_transfer) | **GET** /{apiVersion}/object-storage/transfer | Get Object Storage transfer data
[**post_cancel_object_storage**](ObjectStorageApi.md#post_cancel_object_storage) | **POST** /{apiVersion}/object-storage/cancel | Cancel Object Storage



## get_object_storage_transfer

> models::GetObjectStorageTransfer200Response get_object_storage_transfer(api_version)
Get Object Storage transfer data

The amount of outbound data transfer used by your account's Object Storage buckets. Object Storage adds 1 terabyte of outbound data transfer to your data transfer pool. See the [Object Storage Overview](https://www.linode.com/docs/products/storage/object-storage/#pricing) guide for details on Object Storage transfer quotas.   <<LB>>  ---   - __OAuth scopes__.      ```     object_storage:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetObjectStorageTransfer200Response**](get_object_storage_transfer_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_cancel_object_storage

> serde_json::Value post_cancel_object_storage(api_version)
Cancel Object Storage

Cancel Object Storage on an Account.  __Warning__. This removes all buckets and their contents from your Account. This data is irretrievable once removed.   <<LB>>  ---   - __CLI__.      ```     linode-cli object-storage cancel     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     object_storage:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

