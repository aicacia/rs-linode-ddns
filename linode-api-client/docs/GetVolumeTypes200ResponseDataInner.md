# GetVolumeTypes200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | __Read-only__ The ID representing the volume type. | [optional][readonly]
**label** | Option<**String**> | __Filterable__, __Read-only__ The volume type label is for display purposes only. | [optional][readonly]
**price** | Option<[**models::GetVolumeTypes200ResponseDataInnerPrice**](get_volume_types_200_response_data_inner_price.md)> |  | [optional]
**region_prices** | Option<[**Vec<models::GetLkeTypes200ResponseDataInnerRegionPricesInner>**](get_lke_types_200_response_data_inner_region_prices_inner.md)> |  | [optional]
**transfer** | Option<**i32**> | __Filterable__, __Read-only__ The monthly outbound transfer amount, in MB. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


