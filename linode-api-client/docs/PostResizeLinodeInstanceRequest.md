# PostResizeLinodeInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_auto_disk_resize** | Option<**bool**> | Automatically resize disks when resizing a Linode. When resizing down to a smaller plan your Linode's data must fit within the smaller disk size. | [optional][default to true]
**migration_type** | Option<**String**> | Type of migration used in moving to a new host or Linode type.  `warm`: the Linode will not power down until the migration is complete. Warm migrations are not available for DC migrations.  `cold`: the Linode will be powered down and migrated. When the migration is complete, the Linode will be powered on. | [optional][default to Cold]
**r#type** | **String** | The ID representing the Linode Type. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


