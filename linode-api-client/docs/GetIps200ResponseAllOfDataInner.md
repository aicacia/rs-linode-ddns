# GetIps200ResponseAllOfDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | Option<**String**> | __Filterable__, __Read-only__ The IP address. | [optional][readonly]
**gateway** | Option<**String**> | __Read-only__ The default gateway for this address. | [optional][readonly]
**linode_id** | Option<**i32**> | __Read-only__ The ID of the Linode this address currently belongs to. For IPv4 addresses, this defaults to the Linode that this address was assigned to on creation. IPv4 addresses may be moved using the [Assign IPv4s to Linodes](https://techdocs.akamai.com/linode-api/reference/post-assign-ipv4s) operation. For SLAAC and link-local addresses, this value may not be changed. | [optional][readonly]
**prefix** | Option<**i32**> | __Filterable__, __Read-only__ The number of bits set in the subnet mask. | [optional][readonly]
**public** | Option<**bool**> | __Read-only__ Whether this is a public or private IP address. | [optional][readonly]
**rdns** | Option<**String**> | __Filterable__ The reverse DNS assigned to this address. For public IPv4 addresses, this will be set to a default value provided by Linode if not explicitly set. | [optional]
**region** | Option<**String**> | __Filterable__, __Read-only__ The Region this IP address resides in. | [optional][readonly]
**subnet_mask** | Option<**String**> | __Read-only__ The mask that separates host bits from network bits for this address. | [optional][readonly]
**r#type** | Option<**String**> | __Filterable__, __Read-only__ The type of address this is. | [optional][readonly]
**vpc_nat_1_1** | Option<[**models::GetIps200ResponseAllOfDataInnerVpcNat11**](get_ips_200_response_allOf_data_inner_vpc_nat_1_1.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


