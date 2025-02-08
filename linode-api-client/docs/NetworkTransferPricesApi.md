# \NetworkTransferPricesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_network_transfer_prices**](NetworkTransferPricesApi.md#get_network_transfer_prices) | **GET** /{apiVersion}/network-transfer/prices | List network transfer prices



## get_network_transfer_prices

> models::GetNetworkTransferPrices200Response get_network_transfer_prices(api_version)
List network transfer prices

Returns collection of network transfer prices, including any region-specific rates.   <<LB>>  ---   - __CLI__.      ```     linode-cli network-transfer prices     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetNetworkTransferPrices200Response**](get_network_transfer_prices_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

