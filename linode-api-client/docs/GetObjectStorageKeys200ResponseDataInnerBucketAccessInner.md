# GetObjectStorageKeys200ResponseDataInnerBucketAccessInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bucket_name** | Option<**String**> | The name of the bucket the key can access in the `region`. | [optional]
**cluster** | Option<**String**> | Identifies the legacy cluster where this key can be used. The key grants access to each specified `bucket_name`, based on the `permissions` set. To support backward compatibility, the API generates this value, based on the `region` set for a new key. See [Create an Object Storage key](https://techdocs.akamai.com/linode-api/reference/post-object-storage-keys) for more information. | [optional]
**permissions** | Option<**String**> | The level of access the key grants to the `bucket_name`. Keys with `read_write` access can manage content in the `bucket_name`, while `read_only` can be used to view content. See [Create an Object Storage key(ref:post-object-storage-keys) for more details. | [optional]
**region** | Option<**String**> | The region where the Object Store `bucket_name` resides. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


