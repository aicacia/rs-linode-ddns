# PutLinodeConfigInterface200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | Option<**bool**> | __Read-only__ Returned as `true` if the interface has been booted using the configuration profile to which the interface belongs. | [optional][readonly]
**id** | Option<**i32**> | __Read-only__ The unique ID representing this interface. | [optional][readonly]
**ip_ranges** | Option<**Vec<String>**> | IPv4 CIDR VPC subnet ranges that are routed to this interface. This only applies to interfaces with a `purpose` of `vpc`. Returned as an empty string (`\"\"`) if the interface has a `purpose` of `public` or `vlan`. | [optional]
**ipam_address** | Option<**String**> | The interface's private IP address, in classless inter-domain routing (CIDR) notation. Only applies to interfaces with a `purpose` of `vlan`. Returned as `null` if the interface has a `purpose` of `public` or `vpc`.  > 📘 > > If Network Helper is enabled, the Linode's interface is automatically configured to use this address after the Linode is rebooted. If Network Helper is disabled, enable the address using [manual static IP configuration](https://www.linode.com/docs/guides/manual-network-configuration/). | [optional]
**ipv4** | Option<[**models::PutLinodeConfigInterface200ResponseIpv4**](put_linode_config_interface_200_response_ipv4.md)> |  | [optional]
**label** | Option<**String**> | __Filterable__ The name set for an interface with a `purpose` of `vlan`. Returned as `null` if the interface has a `purpose` of `public` or `vpc`. | [optional]
**primary** | Option<**bool**> | The default route to the Linode. Each Linode can have one interface set as its `primary`. Defaults to the first non-`vlan` type interface, if you haven't specifically set a `primary`.  > 📘 > > The value needs to `false` for any interface that uses `vlan` as its `purpose`. | [optional]
**purpose** | **String** | The type of interface. This can be `public`, `vlan`, or `vpc`.  For interfaces with a `purpose` of `public`:  - You can only define one `public` interface per Linode.  - The Linode's default public IPv4 address is assigned to the `public` interface.  - A Linode needs a `public` interface in the first or `eth0` position to be reachable via the public internet, after it boots. If no `public` interface is configured, you can only access the Linode through [LISH](https://www.linode.com/docs/products/compute/compute-instances/guides/lish/), or through another Linode that's connected to the same VLAN or VPC.  For interfaces with a `purpose` of `vlan`:  - The target Linode is attached to the VLAN with the specified `label`.  - If an `ipam_address` is configured, the Linode uses this address.  For interfaces with a `purpose` of `vpc`:  - The target Linode is attached to an existing VPC subnet with the specified `subnet_id`.  - When the interface is activated, the Linode is configured to use an IP address from the range in the assigned VPC subnet. See `ipv4.vpc` for more information. | 
**subnet_id** | Option<**i32**> | The `id` of the VPC subnet for this interface. Only applies to interfaces with a `purpose` of `vpc`. Returned as `null` if the interface has a `purpose` of `public` or `vlan`.  > 📘 > > You need to reboot a Linode using the interface's configuration profile to assign the Linode to a VPC subnet. | [optional]
**vpc_id** | Option<**i32**> | __Read-only__ The `id` of the VPC configured for this interface. Returned as `null` for non-`vpc` type interfaces. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


