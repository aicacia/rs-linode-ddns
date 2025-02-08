# \LinodeTypesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_linode_type**](LinodeTypesApi.md#get_linode_type) | **GET** /{apiVersion}/linode/types/{typeId} | Get a type
[**get_linode_types**](LinodeTypesApi.md#get_linode_types) | **GET** /{apiVersion}/linode/types | List types



## get_linode_type

> models::GetLinodeTypes200ResponseDataInner get_linode_type(api_version, type_id)
Get a type

Returns information about a specific Linode Type, including pricing and specifications. This is used when [creating](https://techdocs.akamai.com/linode-api/reference/post-linode-instance) or [resizing](https://techdocs.akamai.com/linode-api/reference/post-resize-linode-instance) Linodes.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes type-view g6-standard-2     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**type_id** | **String** | The ID of the Linode Type to look up. | [required] |

### Return type

[**models::GetLinodeTypes200ResponseDataInner**](get_linode_types_200_response_data_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_types

> models::GetLinodeTypes200Response get_linode_types(api_version)
List types

Returns Linode Types, including pricing and specifications for each Type. Use these when [creating](https://techdocs.akamai.com/linode-api/reference/post-linode-instance) or [resizing](https://techdocs.akamai.com/linode-api/reference/post-resize-linode-instance) Linodes.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes types     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetLinodeTypes200Response**](get_linode_types_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

