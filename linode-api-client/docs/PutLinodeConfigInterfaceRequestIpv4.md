# PutLinodeConfigInterfaceRequestIpv4

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**nat_1_1** | Option<[**models::PutLinodeConfigInterfaceRequestIpv4Nat11**](put_linode_config_interface_request_ipv4_nat_1_1.md)> |  | [optional]
**vpc** | Option<**String**> | The VPC subnet IPv4 address for this interface.  - The `vpc` can't be assigned to an existing Linode as an address or in a range.  - The target address can't be the first two or last two addresses in the subnet IPv4 range.  - If omitted, a valid address within the subnet IPv4 range is automatically assigned. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


