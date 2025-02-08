# GetLinodeDisks200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | Option<**String**> | __Read-only__ When this disk was created. | [optional][readonly]
**disk_encryption** | Option<**String**> | __Limited availability__, __Read-only__ Displays if encryption is enabled on this disk. This setting is based on the `disk_encryption` setting of the Linode. | [optional][readonly][default to enabled]
**filesystem** | Option<**String**> | The disk's format or file system. A value of `raw` indicates no file system, just a raw binary stream. A value of `swap` indicates a Linux swap area. The values `ext3` or `ext4` represent these Linux journaling file systems. The value `ext2` is the deprecated ext2 Linux file system. Finally, `initrd` indicates the disk is formatted as an uncompressed initial RAM disk.  > 📘 > > The `ext2` file system doesn't properly support timestamps and will be removed from Linux support in the near future. Also, `initrd` is a legacy format that no longer applies to most use cases. As a best practice, use the other supported formats or file systems instead. | [optional]
**id** | Option<**i32**> | __Read-only__ This disk's ID. You need this value to run other operations that interact with the disk. | [optional][readonly]
**label** | Option<**String**> | __Filterable__ The name of the disk. This is for display purposes only. | [optional]
**size** | Option<**i32**> | __Filterable__ The size of the disk in MB. | [optional]
**status** | Option<**String**> | __Read-only__ The current state of the disk. | [optional][readonly]
**updated** | Option<**String**> | __Read-only__ When this disk was last updated. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


