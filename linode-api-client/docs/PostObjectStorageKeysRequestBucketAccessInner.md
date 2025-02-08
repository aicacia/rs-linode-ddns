# PostObjectStorageKeysRequestBucketAccessInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bucket_name** | Option<**String**> | The `label` set for a bucket that this key grants access to. | [optional]
**permissions** | Option<**String**> | The level of access the key grants to the specified `bucket_name`. Keys with `read_write` access can manage content in the `bucket_name`, while `read_only` can be used to view content. See [Create an Object Storage key]((ref:post-object-storage-keys) for more details. | [optional]
**region** | Option<**String**> | The region where the `bucket_name` you want the key to access is located.  > 📘 > > If you repeat the same `region` across objects, the response includes a single `s3_endpoint` for this region. Use it to access all `bucket_name` entries. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


