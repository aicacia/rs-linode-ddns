# PutObjectStorageKey200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_key** | Option<**String**> | __Read-only__ A unique string chosen by the API to identify this key. Used as a username to identify this key when making requests to the S3 API. | [optional][readonly]
**id** | Option<**i32**> | __Read-only__ This Object Storage key's unique numeric identifier. | [optional][readonly]
**label** | Option<**String**> | The label given to this key. For display purposes only. | [optional]
**limited** | Option<**bool**> | __Read-only__ Whether this Object Storage key limits access to specific buckets and permissions. Returns `false` if this key grants full access.  > 📘 > > The `bucket_access` array that contains limited Object Storage key settings doesn't appear in this response. Store this key's `id` from the response and run [Get an Object Storage key](https://techdocs.akamai.com/linode-api/reference/get-object-storage-key) to view these settings. | [optional][readonly]
**regions** | Option<[**Vec<models::PutObjectStorageKey200ResponseRegionsInner>**](put_object_storage_key_200_response_regions_inner.md)> | The key can be used in these regions to create new buckets, but it can't be used to manage content in those buckets. See [Create an Object Storage key](https://techdocs.akamai.com/linode-api/reference/post-object-storage-keys) for more details. | [optional]
**secret_key** | Option<**String**> | __Read-only__ This Object Storage key's secret key. Used as a password to validate this key when making requests to the S3 API. This value is only revealed in a response after creating or modifying a key. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


