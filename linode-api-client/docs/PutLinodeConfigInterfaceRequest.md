# PutLinodeConfigInterfaceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ip_ranges** | Option<**Vec<String>**> | IPv4 CIDR VPC subnet ranges that are routed to this interface.  - A range can't include any addresses that are assigned to an active Linode or another VPC subnet.  - When updating, you need to include any existing ranges to maintain them. If a range is left out, it will be removed.  - Include this as an empty array (`[]`) to remove all existing `nat_1_1` values.  - Omit this array to leave all existing `ip_ranges` as is. <<LB>> > 📘 > > This only applies to interfaces with a `purpose` of `vpc`. | [optional]
**ipv4** | Option<[**models::PutLinodeConfigInterfaceRequestIpv4**](put_linode_config_interface_request_ipv4.md)> |  | [optional]
**primary** | Option<**bool**> | Set to `true` to label this configuration profile interface as the default route to the Linode.  - Each Linode can have one interface set as its `primary`.  - If you don't specifically set a `primary`, the first non-`vlan` type interface is automatically treated as the primary. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


