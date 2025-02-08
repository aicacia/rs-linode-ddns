# \RecordsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_domain_record**](RecordsApi.md#delete_domain_record) | **DELETE** /{apiVersion}/domains/{domainId}/records/{recordId} | Delete a domain record
[**get_domain_record**](RecordsApi.md#get_domain_record) | **GET** /{apiVersion}/domains/{domainId}/records/{recordId} | Get a domain record
[**get_domain_records**](RecordsApi.md#get_domain_records) | **GET** /{apiVersion}/domains/{domainId}/records | List domain records
[**post_domain_record**](RecordsApi.md#post_domain_record) | **POST** /{apiVersion}/domains/{domainId}/records | Create a domain record
[**put_domain_record**](RecordsApi.md#put_domain_record) | **PUT** /{apiVersion}/domains/{domainId}/records/{recordId} | Update a domain record



## delete_domain_record

> serde_json::Value delete_domain_record(api_version, domain_id, record_id)
Delete a domain record

Deletes a Record on this Domain.   <<LB>>  ---   - __CLI__.      ```     linode-cli domains records-delete 123 234     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     domains:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**domain_id** | **i32** | The ID of the Domain whose Record you are accessing. | [required] |
**record_id** | **i32** | The ID of the Record you are accessing. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domain_record

> models::GetDomainRecords200ResponseDataInner get_domain_record(api_version, domain_id, record_id)
Get a domain record

View a single Record on this Domain.   <<LB>>  ---   - __CLI__.      ```     linode-cli domains records-view 123 234     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     domains:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**domain_id** | **i32** | The ID of the Domain whose Record you are accessing. | [required] |
**record_id** | **i32** | The ID of the Record you are accessing. | [required] |

### Return type

[**models::GetDomainRecords200ResponseDataInner**](get_domain_records_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domain_records

> models::GetDomainRecords200Response get_domain_records(api_version, domain_id, page, page_size)
List domain records

Returns a paginated list of Records configured on a Domain in Linode's DNS Manager.   <<LB>>  ---   - __CLI__.      ```     linode-cli domains records-list 1234     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     domains:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**domain_id** | **i32** | The ID of the Domain we are accessing Records for. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetDomainRecords200Response**](get_domain_records_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_domain_record

> models::GetDomainRecords200ResponseDataInner post_domain_record(api_version, domain_id, post_domain_record_request)
Create a domain record

Adds a new Domain Record to the zonefile this Domain represents.  Each domain can have up to 12,000 active records.   <<LB>>  ---   - __CLI__.      ```     linode-cli domains records-create 123 \\   --type A \\   --name test \\   --target 203.0.113.1 \\   --priority 50 \\   --weight 50 \\   --port 80 \\   --ttl_sec 604800     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     domains:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**domain_id** | **i32** | The ID of the Domain we are accessing Records for. | [required] |
**post_domain_record_request** | [**PostDomainRecordRequest**](PostDomainRecordRequest.md) | Information about the new Record to add. | [required] |

### Return type

[**models::GetDomainRecords200ResponseDataInner**](get_domain_records_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_domain_record

> models::GetDomainRecords200ResponseDataInner put_domain_record(api_version, domain_id, record_id, put_domain_record_request)
Update a domain record

Updates a single Record on this Domain.   <<LB>>  ---   - __CLI__.      ```     linode-cli domains records-update 123 234 \\   --name test \\   --target 203.0.113.1 \\   --priority 50 \\   --weight 50 \\   --port 80 \\   --ttl_sec 604800     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     domains:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**domain_id** | **i32** | The ID of the Domain whose Record you are accessing. | [required] |
**record_id** | **i32** | The ID of the Record you are accessing. | [required] |
**put_domain_record_request** | [**PutDomainRecordRequest**](PutDomainRecordRequest.md) | The values to change. | [required] |

### Return type

[**models::GetDomainRecords200ResponseDataInner**](get_domain_records_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

