# \ClientsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_longview_client**](ClientsApi.md#delete_longview_client) | **DELETE** /{apiVersion}/longview/clients/{clientId} | Delete a Longview client
[**get_longview_client**](ClientsApi.md#get_longview_client) | **GET** /{apiVersion}/longview/clients/{clientId} | Get a Longview client
[**get_longview_clients**](ClientsApi.md#get_longview_clients) | **GET** /{apiVersion}/longview/clients | List Longview clients
[**post_longview_client**](ClientsApi.md#post_longview_client) | **POST** /{apiVersion}/longview/clients | Create a Longview client
[**put_longview_client**](ClientsApi.md#put_longview_client) | **PUT** /{apiVersion}/longview/clients/{clientId} | Update a Longview client



## delete_longview_client

> serde_json::Value delete_longview_client(api_version, client_id)
Delete a Longview client

Deletes a Longview Client from your Account.  __All information stored for this client will be lost.__  This _does not_ uninstall the Longview Client application for your Linode - you must do that manually.   <<LB>>  ---   - __CLI__.      ```     linode-cli longview delete 789     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     longview:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**client_id** | **i32** | The Longview Client ID to access. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_longview_client

> models::GetLongviewClients200ResponseDataInner get_longview_client(api_version, client_id)
Get a Longview client

Returns a single Longview Client you can access.   <<LB>>  ---   - __CLI__.      ```     linode-cli longview view 789     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     longview:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**client_id** | **i32** | The Longview Client ID to access. | [required] |

### Return type

[**models::GetLongviewClients200ResponseDataInner**](get_longview_clients_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_longview_clients

> models::GetLongviewClients200Response get_longview_clients(api_version, page, page_size)
List Longview clients

Returns a paginated list of Longview Clients you have access to. Longview Client is used to monitor stats on your Linode with the help of the Longview Client application.   <<LB>>  ---   - __CLI__.      ```     linode-cli longview list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     longview:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetLongviewClients200Response**](get_longview_clients_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_longview_client

> models::GetLongviewClients200ResponseDataInner post_longview_client(api_version, post_longview_client_request)
Create a Longview client

Creates a Longview Client.  This Client will not begin monitoring the status of your server until you configure the Longview Client application on your Linode using the returning `install_code` and `api_key`.   <<LB>>  ---   - __CLI__.      ```     linode-cli longview create \\   --label client789     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     longview:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_longview_client_request** | [**PostLongviewClientRequest**](PostLongviewClientRequest.md) | Information about the LongviewClient to create. | [required] |

### Return type

[**models::GetLongviewClients200ResponseDataInner**](get_longview_clients_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_longview_client

> models::GetLongviewClients200ResponseDataInner put_longview_client(api_version, client_id, post_longview_client_request)
Update a Longview client

Updates a Longview Client.  This cannot update how it monitors your server; use the Longview Client application on your Linode for monitoring configuration.   <<LB>>  ---   - __CLI__.      ```     linode-cli longview update 789 \\   --label client789     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     longview:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**client_id** | **i32** | The Longview Client ID to access. | [required] |
**post_longview_client_request** | [**PostLongviewClientRequest**](PostLongviewClientRequest.md) | The fields to update. | [required] |

### Return type

[**models::GetLongviewClients200ResponseDataInner**](get_longview_clients_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

