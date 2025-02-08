# Volume

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | Option<**String**> | __Read-only__ When this Volume was created. | [optional][readonly]
**encryption** | Option<**String**> | __Limited availability__, __Read-only__ Displays if encryption is enabled on this volume. | [optional][readonly]
**filesystem_path** | Option<**String**> | __Read-only__ The full filesystem path for the Volume based on the Volume's label. Path is /dev/disk/by-id/scsi-0Linode_Volume_ + Volume label. | [optional][readonly]
**hardware_type** | Option<**String**> | __Read-only__ The storage type of this Volume. | [optional][readonly]
**id** | Option<**i32**> | __Read-only__ The unique ID of this Volume. | [optional][readonly]
**label** | Option<**String**> | __Filterable__ The Volume's label is for display purposes only. | [optional]
**linode_id** | Option<**i32**> | If a Volume is attached to a specific Linode, the ID of that Linode will be displayed here. | [optional]
**linode_label** | Option<**String**> | __Read-only__ If a Volume is attached to a specific Linode, the label of that Linode will be displayed here. | [optional][readonly]
**region** | Option<**String**> | __Read-only__ The unique ID of this Region. | [optional][readonly]
**size** | Option<**i32**> | The Volume's size, in GiB. | [optional]
**status** | Option<**String**> | __Read-only__ The current status of the volume.  Can be one of:    - `creating`. The volume is being created and is not yet available for use.   - `active`. The volume is online and available for use.   - `resizing`. The volume is in the process of upgrading its current capacity.   - `key_rotating`. The volume is in the process of rotating encryption keys. Requests to resize, delete, or clone a volume fails during encryption key rotation. | [optional][readonly]
**tags** | Option<**Vec<String>**> | __Filterable__ An array of Tags applied to this object.  Tags are for organizational purposes only. | [optional]
**updated** | Option<**String**> | __Read-only__ When this Volume was last updated. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


