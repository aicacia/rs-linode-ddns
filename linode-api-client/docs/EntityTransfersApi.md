# \EntityTransfersApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_entity_transfer**](EntityTransfersApi.md#delete_entity_transfer) | **DELETE** /{apiVersion}/account/entity-transfers/{token} | Cancel an entity transfer
[**get_entity_transfer**](EntityTransfersApi.md#get_entity_transfer) | **GET** /{apiVersion}/account/entity-transfers/{token} | Get an entity transfer
[**get_entity_transfers**](EntityTransfersApi.md#get_entity_transfers) | **GET** /{apiVersion}/account/entity-transfers | List entity transfers
[**post_accept_entity_transfer**](EntityTransfersApi.md#post_accept_entity_transfer) | **POST** /{apiVersion}/account/entity-transfers/{token}/accept | Accept an entity transfer
[**post_entity_transfer**](EntityTransfersApi.md#post_entity_transfer) | **POST** /{apiVersion}/account/entity-transfers | Create an entity transfer



## delete_entity_transfer

> serde_json::Value delete_entity_transfer(api_version, token)
Cancel an entity transfer

__Deprecated__ Please run [Cancel a service transfer](https://techdocs.akamai.com/linode-api/reference/delete-service-transfer).   <<LB>>  ---   - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**token** | **uuid::Uuid** | The UUID of the Entity Transfer. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_entity_transfer

> models::GetEntityTransfers200ResponseAllOfDataInner get_entity_transfer(api_version, token)
Get an entity transfer

__Deprecated__ Please run [Get a service transfer request](https://techdocs.akamai.com/linode-api/reference/get-service-transfer).   <<LB>>  ---   - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**token** | **uuid::Uuid** | The UUID of the Entity Transfer. | [required] |

### Return type

[**models::GetEntityTransfers200ResponseAllOfDataInner**](get_entity_transfers_200_response_allOf_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_entity_transfers

> models::GetEntityTransfers200Response get_entity_transfers(api_version, page, page_size)
List entity transfers

__Deprecated__ Please run [List service transfers](https://techdocs.akamai.com/linode-api/reference/get-service-transfers).   <<LB>>  ---   - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetEntityTransfers200Response**](get_entity_transfers_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_accept_entity_transfer

> serde_json::Value post_accept_entity_transfer(api_version, token)
Accept an entity transfer

__Deprecated__ Please run [Accept a service transfer](https://techdocs.akamai.com/linode-api/reference/post-accept-service-transfer).   <<LB>>  ---   - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**token** | **uuid::Uuid** | The UUID of the Entity Transfer. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_entity_transfer

> models::GetEntityTransfers200ResponseAllOfDataInner post_entity_transfer(api_version, post_entity_transfer_request)
Create an entity transfer

__Deprecated__ Please run [Request a service transfer](https://techdocs.akamai.com/linode-api/reference/post-service-transfer).   <<LB>>  ---   - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_entity_transfer_request** | Option<[**PostEntityTransferRequest**](PostEntityTransferRequest.md)> | The entities to include in this transfer request. |  |

### Return type

[**models::GetEntityTransfers200ResponseAllOfDataInner**](get_entity_transfers_200_response_allOf_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

