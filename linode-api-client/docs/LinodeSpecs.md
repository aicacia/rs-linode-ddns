# LinodeSpecs

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**disk** | Option<**i32**> | __Read-only__ The amount of storage space, in MB, this Linode has access to. A typical Linode divides this space between a primary disk with an `image` deployed to it, and a swap disk, usually 512 MB. This is the default configuration created when deploying a Linode with an `image` through [Create a Linode](https://techdocs.akamai.com/linode-api/reference/post-linode-instance). While this configuration is suitable for 99% of use cases, if you need finer control over your Linode's disks, see the [List disks](https://techdocs.akamai.com/linode-api/reference/get-linode-disks) operations. | [optional][readonly]
**gpus** | Option<**i32**> | __Read-only__ The number of GPUs this Linode has access to. | [optional][readonly]
**memory** | Option<**i32**> | __Read-only__ The amount of RAM, in MB, this Linode has access to.  Typically, a Linode boots with all of its available RAM, but this can be configured in a config profile. See the [List config profiles](https://techdocs.akamai.com/linode-api/reference/get-linode-configs) operation for more information. | [optional][readonly]
**transfer** | Option<**i32**> | __Read-only__ The amount of network transfer this Linode is allotted each month. | [optional][readonly]
**vcpus** | Option<**i32**> | __Read-only__ The number of VCPUs this Linode has access to. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


