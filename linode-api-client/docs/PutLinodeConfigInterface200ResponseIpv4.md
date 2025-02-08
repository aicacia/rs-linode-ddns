# PutLinodeConfigInterface200ResponseIpv4

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**nat_1_1** | Option<**String**> | The 1:1 NAT IPv4 address, used to associate a public IPv4 address with the interface's VPC subnet IPv4 address. This only applies to interfaces with a `purpose` of `vpc`. Returned as `null` if no 1:1 NAT is set for a `vpc` interface. Returned as an empty string (`\"\"`) if the interface has a `purpose` of `public` or `vlan`. | [optional]
**vpc** | Option<**String**> | The VPC subnet IPv4 address for this interface. This only applies to interfaces with a `purpose` of `vpc`. Returned as an empty string (`\"\"`) if the interface has a `purpose` of `public` or `vlan`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


