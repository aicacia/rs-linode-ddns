# PostObjectStorageKeysRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bucket_access** | Option<[**Vec<models::PostObjectStorageKeysRequestBucketAccessInner>**](post_object_storage_keys_request_bucket_access_inner.md)> | Set up the key to limit access to specific buckets, each with a specific permission level. You can create a limited Object Storage key with access to no buckets. Include an empty `bucket_access` array in the request. | [optional]
**label** | Option<**String**> | The label for the Object Storage key, for display purposes only. | [optional]
**regions** | Option<**Vec<String>**> | You can use a key to create new buckets in regions set in this array. But it can't be used to manage content in those buckets. See [Create an Object Storage key](https://techdocs.akamai.com/linode-api/reference/post-object-storage-keys) for more details. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


