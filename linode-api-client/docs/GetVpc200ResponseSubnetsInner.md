# GetVpc200ResponseSubnetsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | Option<**String**> | __Filterable__, __Read-only__ The date-time of VPC Subnet creation. | [optional][readonly]
**id** | Option<**i32**> | __Filterable__, __Read-only__ The unique ID of the VPC Subnet. | [optional][readonly]
**ipv4** | Option<**String**> | IPv4 range in CIDR canonical form.  - The range must belong to a private address space as defined in [RFC1918](https://datatracker.ietf.org/doc/html/rfc1918). - Allowed prefix lengths: 1-29. - The range must not overlap with 192.168.128.0/17. - The range must not overlap with other Subnets on the same VPC. | [optional]
**label** | Option<**String**> | __Filterable__ The VPC Subnet's label, for display purposes only.  - Must be unique among the VPC's Subnets. - Can only contain ASCII letters, numbers, and hyphens (`-`). You can't use two consecutive hyphens (`--`). | [optional]
**linodes** | Option<[**Vec<models::GetVpcs200ResponseAllOfDataInnerSubnetsInnerLinodesInner>**](get_vpcs_200_response_allOf_data_inner_subnets_inner_linodes_inner.md)> | __Read-only__ An array of Linode IDs assigned to the VPC Subnet.  A Linode is assigned to a VPC Subnet if it has a Configuration Profile with a `vpc` purpose interface with the subnet's `subnet_id`. Even if the Configuration Profile is not active, meaning the Linode does not have access to the Subnet, the Linode still appears in this array. | [optional][readonly]
**updated** | Option<**String**> | __Filterable__, __Read-only__ The date-time of the most recent VPC Subnet update. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


