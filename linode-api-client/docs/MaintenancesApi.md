# \MaintenancesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_maintenance**](MaintenancesApi.md#get_maintenance) | **GET** /{apiVersion}/account/maintenance | List maintenances



## get_maintenance

> models::GetMaintenance200Response get_maintenance(api_version)
List maintenances

Returns a collection of Maintenance objects for any entity a user has permissions to view. Canceled Maintenance objects are not returned.  Currently, Linodes are the only entities available for viewing.   <<LB>>  ---   - __CLI__.      ```     linode-cli account maintenance-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetMaintenance200Response**](get_maintenance_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

