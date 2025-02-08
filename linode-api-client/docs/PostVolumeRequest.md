# PostVolumeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config_id** | Option<**i32**> | When creating a Volume attached to a Linode, the ID of the Linode Config to include the new Volume in. This Config must belong to the Linode referenced by `linode_id`. Must _not_ be provided if `linode_id` is not sent. If a `linode_id` is sent without a `config_id`, the volume will be attached:    - to the Linode's only config if it only has one config.   - to the Linode's last used config, if possible.  If no config can be selected for attachment, an error will be returned. | [optional]
**encryption** | Option<**String**> | __Limited availability__ Enables encryption on the volume. Full disk encryption ensures the data stored on a block storage volume drive is secure. It protects against unauthorized access by keeping the data encrypted if the volume drive is removed from the data center, decommissioned, or disposed of.  The platform automatically manages the encryption and decryption process for you. You can use an encrypted volume the same way as you use a non-encrypted volume.  You can enable or disable disk encryption only when creating new block storage volumes. After a volume is created, the encryption setting can't be changed. | [optional][default to Disabled]
**label** | **String** | The Volume's label, which is also used in the `filesystem_path` of the resulting volume. | 
**linode_id** | Option<**i32**> | The Linode this volume should be attached to upon creation. If not given, the volume will be created without an attachment. | [optional]
**region** | Option<**String**> | The Region to deploy this Volume in. This is only required if a linode_id is not given. | [optional]
**size** | Option<**i32**> | The initial size of this volume, in GB.  Be aware that volumes may only be resized up after creation. | [optional][default to 20]
**tags** | Option<**Vec<String>**> | __Filterable__ An array of Tags applied to this object.  Tags are for organizational purposes only. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


