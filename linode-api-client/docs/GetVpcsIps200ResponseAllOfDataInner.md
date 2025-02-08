# GetVpcsIps200ResponseAllOfDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | Option<**bool**> | __Filterable__, __Read-only__ Returns `true` if the VPC interface is in use, meaning that the Linode was powered on using the `config_id` to which the interface belongs. Otherwise returns `false`. | [optional][readonly]
**address** | Option<**String**> | __Read-only__ An IPv4 address configured for this VPC interface. These follow the [RFC 1918](https://datatracker.ietf.org/doc/html/rfc1918) private address format. Displayed as `null` if an `address_range`. | [optional][readonly]
**address_range** | Option<**String**> | __Read-only__ A range of IPv4 addresses configured for this VPC interface. Displayed as `null` if a single `address`. | [optional][readonly]
**config_id** | Option<**i32**> | __Filterable__, __Read-only__ The globally general entity identifier for the Linode configuration profile where the VPC is included. | [optional][readonly]
**gateway** | Option<**String**> | __Read-only__ The default gateway for the VPC subnet that the IP or IP range belongs to. | [optional][readonly]
**interface_id** | Option<**i32**> | __Read-only__ The globally general API entity identifier for the Linode interface. | [optional][readonly]
**linode_id** | Option<**i32**> | __Filterable__, __Read-only__ The identifier for the Linode the VPC interface currently belongs to. | [optional][readonly]
**nat_1_1** | Option<**String**> | __Read-only__ The public IP address used for NAT 1:1 with the VPC. This is empty if NAT 1:1 isn't used. | [optional][readonly]
**prefix** | Option<**i32**> | __Read-only__ The number of bits set in the `subnet_mask`. | [optional][readonly]
**region** | Option<**String**> | __Filterable__, __Read-only__ The region of the VPC. | [optional][readonly]
**subnet_id** | Option<**i32**> | The `id` of the VPC Subnet for this interface. | [optional]
**subnet_mask** | Option<**String**> | __Read-only__ The mask that separates host bits from network bits for the `address` or `address_range`. | [optional][readonly]
**vpc_id** | Option<**i32**> | __Filterable__, __Read-only__ The unique globally general API entity identifier for the VPC. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


