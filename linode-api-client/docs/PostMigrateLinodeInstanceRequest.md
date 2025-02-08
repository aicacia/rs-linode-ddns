# PostMigrateLinodeInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**placement_group** | Option<[**models::PostMigrateLinodeInstanceRequestPlacementGroup**](post_migrate_linode_instance_request_placement_group.md)> |  | [optional]
**region** | Option<**String**> | The region to which the Linode will be migrated. Must be a valid region slug. A list of regions can be viewed by running the [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions) operation. A cross data center migration will cancel a pending migration that has not yet been initiated. A cross data center migration will initiate a `linode_migrate_datacenter_create` event. | [optional]
**r#type** | Option<**String**> | Type of migration used in moving to a new host or Linode type.  `warm`: the Linode will not power down until the migration is complete. Warm migrations are not available for DC migrations.  `cold`: the Linode will be powered down and migrated. When the migration is complete, the Linode will be powered on. | [optional][default to Cold]
**upgrade** | Option<**bool**> | When initiating a cross DC migration, setting this value to `true` will also ensure that the Linode is upgraded to the latest generation of hardware that corresponds to your Linode's Type, if any free upgrades are available for it. If no free upgrades are available, and this value is set to `true`, then the endpoint will return a 400 error code and the migration will not be performed. If the data center set in the `region` field does not allow upgrades, then the endpoint will return a 400 error code and the migration will not be performed. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


