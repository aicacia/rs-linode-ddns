# \NetworkTransferStatisticsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_linode_transfer_by_year_month**](NetworkTransferStatisticsApi.md#get_linode_transfer_by_year_month) | **GET** /{apiVersion}/linode/instances/{linodeId}/transfer/{year}/{month} | Get monthly network transfer stats



## get_linode_transfer_by_year_month

> models::GetLinodeTransferByYearMonth200Response get_linode_transfer_by_year_month(api_version, linode_id, year, month)
Get monthly network transfer stats

Returns a Linode's network transfer statistics for a specific month. The year/month values must be either a date in the past, or the current month.   <<LB>>  ---   - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**year** | **i32** | Numeric value representing the year to look up. | [required] |
**month** | **i32** | Numeric value representing the month to look up. | [required] |

### Return type

[**models::GetLinodeTransferByYearMonth200Response**](get_linode_transfer_by_year_month_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

