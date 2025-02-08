# \ServicesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_managed_service**](ServicesApi.md#delete_managed_service) | **DELETE** /{apiVersion}/managed/services/{serviceId} | Delete a managed service
[**get_managed_service**](ServicesApi.md#get_managed_service) | **GET** /{apiVersion}/managed/services/{serviceId} | Get a managed service
[**get_managed_services**](ServicesApi.md#get_managed_services) | **GET** /{apiVersion}/managed/services | List managed services
[**post_disable_managed_service**](ServicesApi.md#post_disable_managed_service) | **POST** /{apiVersion}/managed/services/{serviceId}/disable | Disable a managed service
[**post_enable_managed_service**](ServicesApi.md#post_enable_managed_service) | **POST** /{apiVersion}/managed/services/{serviceId}/enable | Enable a managed service
[**post_managed_service**](ServicesApi.md#post_managed_service) | **POST** /{apiVersion}/managed/services | Create a managed service
[**put_managed_service**](ServicesApi.md#put_managed_service) | **PUT** /{apiVersion}/managed/services/{serviceId} | Update a managed service



## delete_managed_service

> serde_json::Value delete_managed_service(api_version, service_id)
Delete a managed service

Deletes a Managed Service.  This service will no longer be monitored by Linode Managed.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed service-delete 9994     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**service_id** | **i32** | The ID of the Managed Service to access. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_managed_service

> models::GetManagedServices200ResponseDataInner get_managed_service(api_version, service_id)
Get a managed service

Returns information about a single Managed Service on your Account.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed service-view 9994     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**service_id** | **i32** | The ID of the Managed Service to access. | [required] |

### Return type

[**models::GetManagedServices200ResponseDataInner**](get_managed_services_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_managed_services

> models::GetManagedServices200Response get_managed_services(api_version)
List managed services

Returns a paginated list of Managed Services on your Account. These are the services Linode Managed is monitoring and will report and attempt to resolve issues with.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed services-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetManagedServices200Response**](get_managed_services_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_disable_managed_service

> models::GetManagedServices200ResponseDataInner post_disable_managed_service(api_version, service_id)
Disable a managed service

Temporarily disables monitoring of a Managed Service.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed service-disable 9994     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**service_id** | **i32** | The ID of the Managed Service to disable. | [required] |

### Return type

[**models::GetManagedServices200ResponseDataInner**](get_managed_services_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_enable_managed_service

> models::GetManagedServices200ResponseDataInner post_enable_managed_service(api_version, service_id)
Enable a managed service

Enables monitoring of a Managed Service.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed service-enable 9994     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**service_id** | **i32** | The ID of the Managed Service to enable. | [required] |

### Return type

[**models::GetManagedServices200ResponseDataInner**](get_managed_services_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_managed_service

> models::GetManagedServices200ResponseDataInner post_managed_service(api_version, post_managed_service_request)
Create a managed service

Creates a Managed Service. Linode Managed will begin monitoring this service and reporting and attempting to resolve any Issues.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed service-create \\   --service_type url \\   --label prod-1 \\   --address \"https://example.org\" \\   --timeout 30 \\   --body \"it worked\" \\   --consultation_group on-call \\   --notes \"The service name is \\     my-cool-application\" \\   --credentials 9991     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_managed_service_request** | Option<[**PostManagedServiceRequest**](PostManagedServiceRequest.md)> | Information about the service to monitor. |  |

### Return type

[**models::GetManagedServices200ResponseDataInner**](get_managed_services_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_managed_service

> models::GetManagedServices200ResponseDataInner put_managed_service(api_version, service_id, put_managed_service_request)
Update a managed service

Updates information about a Managed Service.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed service-update 9994 \\   --service_type url \\   --label prod-1 \\   --address \"https://example.org\" \\   --timeout 30 \\   --body \"it worked\" \\   --consultation_group on-call \\   --notes \"The service name is my-cool-application\" \\   --credentials 9991     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**service_id** | **i32** | The ID of the Managed Service to access. | [required] |
**put_managed_service_request** | [**PutManagedServiceRequest**](PutManagedServiceRequest.md) | The fields to update. | [required] |

### Return type

[**models::GetManagedServices200ResponseDataInner**](get_managed_services_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

