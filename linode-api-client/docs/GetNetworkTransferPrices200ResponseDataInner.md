# GetNetworkTransferPrices200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | __Read-only__ The ID representing the network transfer price. | [optional][readonly]
**label** | Option<**String**> | __Filterable__, __Read-only__ The network transfer price label is for display purposes only. | [optional][readonly]
**price** | Option<[**models::GetNetworkTransferPrices200ResponseDataInnerPrice**](get_network_transfer_prices_200_response_data_inner_price.md)> |  | [optional]
**region_prices** | Option<[**Vec<models::GetNetworkTransferPrices200ResponseDataInnerRegionPricesInner>**](get_network_transfer_prices_200_response_data_inner_region_prices_inner.md)> |  | [optional]
**transfer** | Option<**i32**> | __Filterable__, __Read-only__ The monthly outbound transfer amount, in MB. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


