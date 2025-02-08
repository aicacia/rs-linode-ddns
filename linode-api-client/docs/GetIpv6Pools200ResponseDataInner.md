# GetIpv6Pools200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**prefix** | Option<**i32**> | The prefix length of the address. The total number of addresses that can be assigned from this range is calculated as 2<sup>(128 - prefix length)</sup>. | [optional]
**range** | Option<**String**> | __Read-only__ The IPv6 range of addresses in this pool. | [optional][readonly]
**region** | Option<**String**> | __Filterable__, __Read-only__ The region for this pool of IPv6 addresses. | [optional][readonly]
**route_target** | Option<**String**> | The last address in this block of IPv6 addresses. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


