# GetIpv6Range200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_bgp** | Option<**bool**> | __Read-only__ Whether this IPv6 range is shared. | [optional][readonly]
**linodes** | Option<**Vec<i32>**> | __Read-only__ A list of Linodes targeted by this IPv6 range. Includes Linodes with IP sharing. | [optional][readonly]
**prefix** | Option<**i32**> | The prefix length of the address. The total number of addresses that can be assigned from this range is calculated as 2<sup>(128 - prefix length)</sup>. | [optional]
**range** | Option<**String**> | __Read-only__ The IPv6 address of this range. | [optional][readonly]
**region** | Option<**String**> | __Read-only__ The region for this range of IPv6 addresses. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


