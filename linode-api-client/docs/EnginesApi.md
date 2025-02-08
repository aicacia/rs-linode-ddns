# \EnginesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_databases_engine**](EnginesApi.md#get_databases_engine) | **GET** /{apiVersion}/databases/engines/{engineId} | Get a Managed Databases engine
[**get_databases_engines**](EnginesApi.md#get_databases_engines) | **GET** /{apiVersion}/databases/engines | List Managed Databases engines



## get_databases_engine

> models::GetDatabasesEngines200ResponseAllOfDataInner get_databases_engine(api_version, engine_id, page, page_size)
Get a Managed Databases engine

Display information for a single Managed Databases engine type and version. Run the [List Managed Databases engines](https://techdocs.akamai.com/linode-api/reference/get-databases-engines) operation and store the `id` for the applicable database engine.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases engine-view     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**engine_id** | **String** | The ID of the Managed Database engine. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetDatabasesEngines200ResponseAllOfDataInner**](get_databases_engines_200_response_allOf_data_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_databases_engines

> models::GetDatabasesEngines200Response get_databases_engines(api_version, page, page_size)
List Managed Databases engines

Display all available Managed Databases engine types and versions. Use an engine's `id` to create a new Managed Databases instance.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases engines     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetDatabasesEngines200Response**](get_databases_engines_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

