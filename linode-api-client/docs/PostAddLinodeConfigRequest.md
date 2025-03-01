# PostAddLinodeConfigRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**comments** | Option<**String**> | Optional field for arbitrary user comments on this configuration. | [optional]
**devices** | [**models::GetLinodeConfigs200ResponseDataInnerDevices**](get_linode_configs_200_response_data_inner_devices.md) |  | 
**helpers** | Option<[**models::GetLinodeConfigs200ResponseDataInnerHelpers**](get_linode_configs_200_response_data_inner_helpers.md)> |  | [optional]
**id** | Option<**i32**> | __Read-only__ The ID of this Config. | [optional][readonly]
**interfaces** | Option<[**Vec<models::PostAddLinodeConfigRequestAllOfInterfacesInner>**](post_add_linode_config_request_allOf_interfaces_inner.md)> | An array of Network Interfaces to add to this Linode's Configuration Profile. At least one and up to three Interface objects can exist in this array. The position in the array determines which of the Linode's network Interfaces is configured:  - First [0]:  eth0 - Second [1]: eth1 - Third [2]:  eth2  When updating a Linode's Interfaces, _each Interface must be redefined_. An empty `interfaces` array results in a default `public` type Interface configuration only.  If no public Interface is configured, public IP addresses are still assigned to the Linode but will not be usable without manual configuration.  __Note__. Changes to Linode Interface configurations can be enabled by rebooting the Linode.  `vpc` details  See the [VPC documentation](https://www.linode.com/docs/products/networking/vpc/#technical-specifications) guide for its specifications and limitations.  `vlan` details  - Only Next Generation Network (NGN) data centers support VLANs. Run the [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions) operation to view the capabilities of data center regions. If a VLAN is attached to your Linode and you attempt to migrate or clone it to a non-NGN data center, the migration or cloning will not initiate. If a Linode cannot be migrated or cloned because of an incompatibility, you will be prompted to select a different data center or contact support. - See the [VLANs Overview](https://www.linode.com/docs/products/networking/vlans/#technical-specifications) guide to view additional specifications and limitations. | [optional]
**kernel** | Option<**String**> | The ID of the kernel used to boot a Linode. Run the [List kernels](https://techdocs.akamai.com/linode-api/reference/get-kernels) operation to see all available kernels. Here are some commonly used kernels:  - `linode/latest-64bit`. This is the default, our latest kernel at the time of an instance boot or reboot.  - `linode/grub2`. The upstream distribution-supplied kernel that's installed on the primary disk, or a custom kernel if installed.  - `linode/direct-disk`. The master boot record (MBR) of the primary disk or root device. Use this in place of a Linux kernel. | [optional][default to linode/latest-64bit]
**label** | **String** | __Filterable__ The name of the configuration for display in Akamai Cloud Manager. | 
**memory_limit** | Option<**i32**> | Defaults to the total RAM of the Linode. | [optional]
**root_device** | Option<**String**> | The root device to boot.  > 📘  - If you leave this empty or set an invalid value, the root device defaults to `/dev/sda`.  - If you specify a device at the root device location and it's not mounted, the Linode won't boot until a device is mounted. | [optional]
**run_level** | Option<**String**> | Defines the state of your Linode after booting. Defaults to `default`. | [optional]
**virt_mode** | Option<**String**> | Controls the virtualization mode. Defaults to `paravirt`.  - `paravirt` is suitable for most cases. Linodes running in `paravirt` mode share some qualities with the host, ultimately making it run faster since there is less transition between it and the host.  - `fullvirt` affords more customization, but is slower because 100% of the VM is virtualized. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


