# \DomainZoneFileApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_domain_zone**](DomainZoneFileApi.md#get_domain_zone) | **GET** /{apiVersion}/domains/{domainId}/zone-file | Get a domain zone file



## get_domain_zone

> models::GetDomainZone200Response get_domain_zone(api_version, domain_id)
Get a domain zone file

Returns the zone file for the last rendered zone for the specified domain.   <<LB>>  ---   - __CLI__.      ```     linode-cli domains zone-file 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     domains:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**domain_id** | **String** | ID of the Domain. | [required] |

### Return type

[**models::GetDomainZone200Response**](get_domain_zone_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

