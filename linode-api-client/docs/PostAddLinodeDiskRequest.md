# PostAddLinodeDiskRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authorized_keys** | Option<**Vec<String>**> | __Write-only__ A list of public SSH keys that will be automatically appended to the root user's `~/.ssh/authorized_keys` file when deploying from an Image. | [optional]
**authorized_users** | Option<**Vec<String>**> | __Write-only__ A list of usernames. If the usernames have associated SSH keys, the keys will be appended to the root users `~/.ssh/authorized_keys` file automatically when deploying from an Image. | [optional]
**filesystem** | Option<**String**> | The disk's format or file system. A value of `raw` indicates no file system, just a raw binary stream. A value of `swap` indicates a Linux swap area. The values `ext3` or `ext4` represent these Linux journaling file systems. The value `ext2` is the deprecated ext2 Linux file system. Finally, `initrd` indicates the disk is formatted as an uncompressed initial RAM disk.  > 📘 > > The `ext2` file system doesn't properly support timestamps and will be removed from Linux support in the near future. Also, `initrd` is a legacy format that no longer applies to most use cases. As a best practice, use the other supported formats or file systems instead. | [optional]
**image** | Option<**String**> | An Image ID to deploy the Linode Disk from.  Run the [List images](https://techdocs.akamai.com/linode-api/reference/get-images) operation with authentication to view all available Images. Official Linode Images start with `linode/`, while your Account's Images start with `private/`. Creating a disk from a Private Image requires `read_only` or `read_write` permissions for that Image. Run the [Update a user's grants](https://techdocs.akamai.com/linode-api/reference/put-user-grants) operation to adjust permissions for an Account Image. | [optional]
**label** | Option<**String**> | __Filterable__ The name of the disk. This is for display purposes only. | [optional]
**root_pass** | Option<**String**> | __Write-only__ This sets the root user's password on a newly created Linode Disk when deploying from an Image.  - __Required__ when creating a Linode Disk from an Image, including when using a StackScript.  - Must meet a password strength score requirement that is calculated internally by the API. If the strength requirement is not met, you will receive a `Password does not meet strength requirement` error. | [optional]
**size** | **i32** | __Filterable__ The size of the Disk in MB.  Images require a minimum size. Run the [Get an image](https://techdocs.akamai.com/linode-api/reference/get-image) operation to view its size. | 
**stackscript_data** | Option<[**serde_json::Value**](.md)> | This field is required only if the StackScript being deployed requires input data from the User for successful completion. See [User Defined Fields (UDFs)](https://www.linode.com/docs/products/tools/stackscripts/guides/write-a-custom-script/#declare-user-defined-fields-udfs) for more details.  This field is required to be valid JSON.  Total length cannot exceed 65,535 characters. | [optional]
**stackscript_id** | Option<**i32**> | A StackScript ID that will cause the referenced StackScript to be run during deployment of this Linode. A compatible `image` is required to use a StackScript. To get a list of available StackScript and their permitted Images, run [List StackScripts](https://techdocs.akamai.com/linode-api/reference/get-stack-scripts). This field cannot be used when deploying from a Backup or a Private Image. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


