# PostImageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cloud_init** | Option<**bool**> | Whether this image supports [cloud-init](https://www.linode.com/docs/guides/write-files-with-cloud-init/). | [optional]
**description** | Option<**String**> | A detailed description of this image. | [optional]
**disk_id** | **i32** | The ID of the target Linode disk for this image. | 
**label** | Option<**String**> | The short title for this image. If not provided, the disk's current label is used. | [optional]
**tags** | Option<**Vec<String>**> | Tags used for organizational purposes. A tag can be from 3 to 100 characters long, and an image can have a maximum of 500 total tags. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


