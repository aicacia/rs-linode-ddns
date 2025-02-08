# PostRebuildLinodeInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authorized_keys** | Option<**Vec<String>**> | __Write-only__ A list of public SSH keys that will be automatically appended to the root user's `~/.ssh/authorized_keys` file when deploying from an Image. | [optional]
**authorized_users** | Option<**Vec<String>**> | __Write-only__ A list of usernames. If the usernames have associated SSH keys, the keys will be appended to the root users `~/.ssh/authorized_keys` file automatically when deploying from an Image. | [optional]
**booted** | Option<**bool**> | __Write-only__ This field defaults to `true` if the Linode is created with an Image or from a Backup. If it is deployed from an Image or a Backup and you wish it to remain `offline` after deployment, set this to `false`. | [optional][default to true]
**disk_encryption** | Option<**String**> | __Limited availability__ Local disk encryption ensures that your data stored on Linodes is secured. Disk encryption protects against unauthorized data access by keeping the data encrypted if the disk is ever removed from the data center, decommissioned, or disposed of. The platform manages the encryption and decryption for you.  By default, encryption is `enabled` on all Linodes. If you opted out of encryption or if the Linode was created prior to local disk encryption support, you can encrypt your data using [Rebuild](https://techdocs.akamai.com/linode-api/reference/post-rebuild-linode-instance). | [optional]
**image** | **String** | An Image ID to deploy the Linode Disk from.  Run the [List images](https://techdocs.akamai.com/linode-api/reference/get-images) operation with authentication to view all available Images. Official Linode Images start with `linode/`, while your Account's Images start with `private/`. Creating a disk from a Private Image requires `read_only` or `read_write` permissions for that Image. Run the [Update a user's grants](https://techdocs.akamai.com/linode-api/reference/put-user-grants) operation to adjust permissions for an Account Image. | 
**metadata** | Option<[**models::PostLinodeInstanceRequestAllOfMetadata**](post_linode_instance_request_allOf_metadata.md)> |  | [optional]
**root_pass** | **String** | __Write-only__ This sets the root user's password on a newly created Linode Disk when deploying from an Image.  - __Required__ when creating a Linode Disk from an Image, including when using a StackScript.  - Must meet a password strength score requirement that is calculated internally by the API. If the strength requirement is not met, you will receive a `Password does not meet strength requirement` error. | 
**stackscript_data** | Option<[**serde_json::Value**](.md)> | This field is required only if the StackScript being deployed requires input data from the User for successful completion. See [User Defined Fields (UDFs)](https://www.linode.com/docs/products/tools/stackscripts/guides/write-a-custom-script/#declare-user-defined-fields-udfs) for more details.  This field is required to be valid JSON.  Total length cannot exceed 65,535 characters. | [optional]
**stackscript_id** | Option<**i32**> | A StackScript ID that will cause the referenced StackScript to be run during deployment of this Linode. A compatible `image` is required to use a StackScript. To get a list of available StackScript and their permitted Images, run [List StackScripts](https://techdocs.akamai.com/linode-api/reference/get-stack-scripts). This field cannot be used when deploying from a Backup or a Private Image. | [optional]
**r#type** | Option<**String**> | The ID of the [Linode type](https://techdocs.akamai.com/linode-api/reference/get-linode-types) to resize to with this request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


