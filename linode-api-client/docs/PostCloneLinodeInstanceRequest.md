# PostCloneLinodeInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backups_enabled** | Option<**bool**> | If this field is set to `true`, the created Linode will automatically be enrolled in the Linode Backup service. This will incur an additional charge. Pricing is included in the response from [List types](https://techdocs.akamai.com/linode-api/reference/get-linode-types).  - Can only be included when cloning to a new Linode. | [optional]
**configs** | Option<**Vec<i32>**> | An array of configuration profile IDs.  - If the `configs` parameter __is not provided__, then __all configuration profiles and their associated disks will be cloned__ from the source Linode. Any disks specified by the `disks` parameter will also be cloned. - __If an empty array is provided__ for the `configs` parameter, then __no configuration profiles (nor their associated disks) will be cloned__ from the source Linode. Any disks specified by the `disks` parameter will still be cloned. - __If a non-empty array is provided__ for the `configs` parameter, then __the configuration profiles specified in the array (and their associated disks) will be cloned__ from the source Linode. Any disks specified by the `disks` parameter will also be cloned. | [optional]
**disks** | Option<**Vec<i32>**> | An array of disk IDs.  - If the `disks` parameter __is not provided__, then __no extra disks will be cloned__ from the source Linode. All disks associated with the configuration profiles specified by the `configs` parameter will still be cloned. - __If an empty array is provided__ for the `disks` parameter, then __no extra disks will be cloned__ from the source Linode. All disks associated with the configuration profiles specified by the `configs` parameter will still be cloned. - __If a non-empty array is provided__ for the `disks` parameter, then __the disks specified in the array will be cloned__ from the source Linode, in addition to any disks associated with the configuration profiles specified by the `configs` parameter. | [optional]
**group** | Option<**String**> | A label used to group Linodes for display. Linodes are not required to have a group. | [optional]
**label** | Option<**String**> | The label to assign this Linode when cloning to a new Linode.  - Can only be provided when cloning to a new Linode. - Defaults to `linode`. | [optional]
**linode_id** | Option<**i32**> | If an existing Linode is the target for the clone, the ID of that Linode. The existing Linode must have enough resources to accept the clone. | [optional]
**metadata** | Option<[**models::PostLinodeInstanceRequestAllOfMetadata**](post_linode_instance_request_allOf_metadata.md)> |  | [optional]
**placement_group** | Option<[**models::PostCloneLinodeInstanceRequestPlacementGroup**](post_clone_linode_instance_request_placement_group.md)> |  | [optional]
**private_ip** | Option<**bool**> | If `true`, the created Linode will have private networking enabled and assigned a private IPv4 address.  - Can only be provided when cloning to a new Linode. | [optional]
**region** | Option<**String**> | This is the Region where the Linode will be deployed. To view all available Regions you can deploy to, run [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions).  - Region can only be provided and is required when cloning to a new Linode. | [optional]
**r#type** | Option<**String**> | A Linode's Type determines what resources are available to it, including disk space, memory, and virtual cpus. The amounts available to a specific Linode are returned as `specs` on the Linode object.  To view all available Linode Types you can deploy with, run [List types](https://techdocs.akamai.com/linode-api/reference/get-linode-types).  - Type can only be provided and is required when cloning to a new Linode. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


