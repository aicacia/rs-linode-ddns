# GetRegions200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**capabilities** | Option<**Vec<String>**> | __Read-only__ A list of capabilities of this region. | [optional][readonly]
**country** | Option<**String**> | __Filterable__, __Read-only__ The country where this Region resides. | [optional][readonly]
**id** | Option<**String**> | __Read-only__ The unique ID of this Region. | [optional][readonly]
**label** | Option<**String**> | __Read-only__ Detailed location information for this Region, including city, state or region, and country. | [optional][readonly]
**placement_group_limits** | Option<[**models::GetRegions200ResponseDataInnerPlacementGroupLimits**](get_regions_200_response_data_inner_placement_group_limits.md)> |  | [optional]
**resolvers** | Option<[**models::GetRegions200ResponseDataInnerResolvers**](get_regions_200_response_data_inner_resolvers.md)> |  | [optional]
**site_type** | Option<**String**> | __Filterable__, __Read-only__ This region's site type. A `core` region indicates a traditional cloud computing [region](https://www.linode.com/docs/products/platform/get-started/guides/choose-a-data-center/#product-availability) that offers all compute services. A `distributed` region indicates sites that are globally dispersed to be closer to end users and workloads. These regions offer limited services. | [optional][readonly]
**status** | Option<**String**> | __Read-only__ This region's current operational status. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


