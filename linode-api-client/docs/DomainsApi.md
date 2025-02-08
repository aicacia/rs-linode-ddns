# \DomainsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_domain**](DomainsApi.md#delete_domain) | **DELETE** /{apiVersion}/domains/{domainId} | Delete a domain
[**get_domain**](DomainsApi.md#get_domain) | **GET** /{apiVersion}/domains/{domainId} | Get a domain
[**get_domains**](DomainsApi.md#get_domains) | **GET** /{apiVersion}/domains | List domains
[**post_clone_domain**](DomainsApi.md#post_clone_domain) | **POST** /{apiVersion}/domains/{domainId}/clone | Clone a domain
[**post_domain**](DomainsApi.md#post_domain) | **POST** /{apiVersion}/domains | Create a domain
[**post_import_domain**](DomainsApi.md#post_import_domain) | **POST** /{apiVersion}/domains/import | Import a domain
[**put_domain**](DomainsApi.md#put_domain) | **PUT** /{apiVersion}/domains/{domainId} | Update a domain



## delete_domain

> serde_json::Value delete_domain(api_version, domain_id)
Delete a domain

Deletes a Domain from Linode's DNS Manager. The Domain will be removed from Linode's nameservers shortly after this operation completes. This also deletes all associated Domain Records.   <<LB>>  ---   - __CLI__.      ```     linode-cli domains delete 1234     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     domains:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**domain_id** | **i32** | The ID of the Domain to access. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domain

> models::Domain get_domain(api_version, domain_id)
Get a domain

This is a single Domain that you have registered in Linode's DNS Manager. Linode is not a registrar, and in order for this Domain record to work you must own the domain and point your registrar at Linode's nameservers.   <<LB>>  ---   - __CLI__.      ```     linode-cli domains view 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     domains:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**domain_id** | **i32** | The ID of the Domain to access. | [required] |

### Return type

[**models::Domain**](Domain.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domains

> models::GetDomains200Response get_domains(api_version, page, page_size)
List domains

This is a collection of Domains that you have registered in Linode's DNS Manager.  Linode is not a registrar, and in order for these to work you must own the domains and point your registrar at Linode's nameservers.   <<LB>>  ---   - __CLI__.      ```     linode-cli domains list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     domains:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetDomains200Response**](get_domains_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_clone_domain

> models::Domain post_clone_domain(api_version, domain_id, post_clone_domain_request)
Clone a domain

Clones a Domain and all associated DNS records from a Domain that is registered in Linode's DNS manager.   <<LB>>  ---   - __CLI__.      ```     linode-cli domains clone 123 --domain example.com     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     domains:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**domain_id** | **String** | ID of the Domain to clone. | [required] |
**post_clone_domain_request** | [**PostCloneDomainRequest**](PostCloneDomainRequest.md) | Information about the Domain to clone. | [required] |

### Return type

[**models::Domain**](Domain.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_domain

> models::Domain post_domain(api_version, post_domain_request)
Create a domain

Adds a new Domain to Linode's DNS Manager. Linode is not a registrar, and you must own the domain before adding it here. Be sure to point your registrar to Linode's nameservers so that the records hosted here are used.   <<LB>>  ---   - __CLI__.      ```     linode-cli domains create \\   --type master \\   --domain example.org \\   --soa_email admin@example.org     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     domains:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_domain_request** | [**PostDomainRequest**](PostDomainRequest.md) | Information about the domain you are registering. | [required] |

### Return type

[**models::Domain**](Domain.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_import_domain

> models::Domain post_import_domain(api_version, post_import_domain_request)
Import a domain

Imports a domain zone from a remote nameserver. Your nameserver must allow zone transfers (AXFR) from the following IPs:  - 96.126.114.97 - 96.126.114.98 - 2600:3c00::5e - 2600:3c00::5f   <<LB>>  ---   - __CLI__.      ```     linode-cli domains import --domain example.com --remote_nameserver examplenameserver.com     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     domains:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_import_domain_request** | Option<[**PostImportDomainRequest**](PostImportDomainRequest.md)> | Information about the Domain to import. |  |

### Return type

[**models::Domain**](Domain.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_domain

> models::Domain put_domain(api_version, domain_id, domain1)
Update a domain

Update information about a Domain in Linode's DNS Manager.   <<LB>>  ---   - __CLI__.      ```     linode-cli domains update 1234 \\   --retry_sec 7200 \\   --ttl_sec 300     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     domains:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**domain_id** | **i32** | The ID of the Domain to access. | [required] |
**domain1** | [**Domain1**](Domain1.md) | The Domain information to update. | [required] |

### Return type

[**models::Domain**](Domain.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

