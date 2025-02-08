# GetIpv6Ranges200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**prefix** | Option<**i32**> | The prefix length of the address. The total number of addresses that can be assigned from this range is calculated as 2<sup>(128 - prefix length)</sup>. | [optional]
**range** | Option<**String**> | __Read-only__ The IPv6 address of this range. | [optional][readonly]
**region** | Option<**String**> | __Read-only__ The region for this range of IPv6 addresses. | [optional][readonly]
**route_target** | Option<**String**> | The IPv6 SLAAC address. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


