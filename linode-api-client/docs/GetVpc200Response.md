# GetVpc200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | Option<**String**> | __Filterable__, __Read-only__ The date-time of VPC creation. | [optional][readonly]
**description** | Option<**String**> | A written description to help distinguish the VPC. | [optional][default to ]
**id** | Option<**i32**> | __Filterable__, __Read-only__ The unique ID of the VPC. | [optional][readonly]
**label** | Option<**String**> | __Filterable__ The VPC's label, for display purposes only.  - Needs to be unique among the Account's VPCs. - Can only contain ASCII letters, numbers, and hyphens (`-`). You can't use two consecutive hyphens (`--`). | [optional]
**region** | Option<**String**> | __Filterable__ The Region for the VPC. | [optional]
**subnets** | Option<[**Vec<models::GetVpc200ResponseSubnetsInner>**](get_vpc_200_response_subnets_inner.md)> | A list of subnets associated with the VPC. | [optional]
**updated** | Option<**String**> | __Filterable__, __Read-only__ The date-time of the most recent VPC update. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


