# PostObjectStorageBucketRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acl** | Option<**String**> | The Access Control Level of the bucket using a canned ACL string. For more fine-grained control of ACLs, use the S3 API directly. | [optional][default to Private]
**cors_enabled** | Option<**bool**> | If set to `false`, CORS is disabled for all origins in the bucket. For more fine-grained controls of CORS, use the S3 API directly. | [optional]
**endpoint_type** | Option<**String**> | The type of `s3_endpoint` available to the active `user` in this `region`. See [Endpoint types](https://techdocs.akamai.com/cloud-computing/docs/object-storage#endpoint-type) for more information. | [optional]
**label** | **String** | The name for this bucket.  * A bucket name can contain from 3 to 63 alphanumeric characters, dashes (`-`), or dots (`.`). * A bucket name can't end in a dash and you can't use two consecutive dashes. * A bucket name can't start or end in a dot, and you can't use two consecutive dots. As a best practice, only use dots if a certificate you're using with your bucket requires it. (For example, if you're using a custom TLS certificate.) * A bucket name needs to be unique in the `region` where you're creating the bucket. The API only reserves labels for the `region` where active buckets are created and stored. If you want to reserve this bucket's label in another `region`, create a new bucket with the same label in the new `region`. | 
**region** | Option<**String**> | The `id` assigned to the data center ([region](https://techdocs.akamai.com/linode-api/reference/get-regions)) where this Object Storage bucket should be created.  > 📘 > > This supports legacy `clusterId` values that represented a specific region. For example, `us-east-1` is the legacy reference for the `us-east` region. | [optional]
**s3_endpoint** | Option<**String**> | The active user's s3 endpoint URL, based on the `endpoint_type` and `region`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


