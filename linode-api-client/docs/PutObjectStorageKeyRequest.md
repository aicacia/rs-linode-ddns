# PutObjectStorageKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**label** | Option<**String**> | The label for the Object Storage key. Used for display purposes. Omit this to leave the `label` unchanged. | [optional]
**regions** | Option<**Vec<String>**> | Replace the current list of `regions` set in a specific key. Include an existing region to maintain it or leave it out to remove it. If you include new `regions` in the key, they can't be used to manage content in buckets in that specific region. You can grant these keys this access using [bucket policies](https://www.linode.com/docs/products/storage/object-storage/guides/bucket-policies/). Omit this to leave the existing list unchanged.  > 🚧 > > You can't remove a `region` from a limited key's original `bucket_access` list. If you include the `regions` array in this operation, you need to include all existing `region` entries from the `bucket_access` array. Otherwise, the operation fails with an error. Run [Get an Object Storage key](https://techdocs.akamai.com/linode-api/reference/get-object-storage-key) to review current `region` entries in a limited key. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


