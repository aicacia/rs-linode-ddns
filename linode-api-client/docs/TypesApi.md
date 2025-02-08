# \TypesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_databases_type**](TypesApi.md#get_databases_type) | **GET** /{apiVersion}/databases/types/{typeId} | Get a Managed Databases type
[**get_databases_types**](TypesApi.md#get_databases_types) | **GET** /{apiVersion}/databases/types | List Managed Databases types



## get_databases_type

> models::GetDatabasesTypes200ResponseAllOfDataInner get_databases_type(api_version, type_id, page, page_size)
Get a Managed Databases type

Display the details of a single Managed Databases node type. The type and number of nodes determine the resources and price of a Managed Databases instance. Run the [List Managed Databases type](https://techdocs.akamai.com/linode-api/reference/get-databases-types) operation and store the `id` for the applicable database node type.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases type-view     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**type_id** | **String** | The ID of the Managed Database type. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetDatabasesTypes200ResponseAllOfDataInner**](get_databases_types_200_response_allOf_data_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_databases_types

> models::GetDatabasesTypes200Response get_databases_types(api_version, page, page_size)
List Managed Databases types

Display all Managed Databases node types. The type and number of nodes determine the resources and price of a Managed Databases instance. Each database can have one node type. With a high availability database, all nodes are deployed according to the chosen type.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases types     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetDatabasesTypes200Response**](get_databases_types_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

