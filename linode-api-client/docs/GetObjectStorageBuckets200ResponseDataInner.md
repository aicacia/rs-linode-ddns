# GetObjectStorageBuckets200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cluster** | Option<**String**> | __Deprecated__ The legacy `clusterId` equivalent for the `regionId` where this bucket lives. The API maintains this for backward compatibility.  > 📘 > > - This value and the `regionId` are interchangeable when used in requests. Best practice is to use the `regionId`. > > - This value is empty for newer regions that don't have a legacy `clusterId`. | [optional]
**created** | Option<**String**> | When this bucket was created. | [optional]
**endpoint_type** | Option<**String**> | The type of `s3_endpoint` available to the active `user` in this `region`. See [Endpoint types](https://techdocs.akamai.com/cloud-computing/docs/object-storage#endpoint-type) for more information. | [optional]
**hostname** | Option<**String**> | The hostname where this bucket can be accessed. This hostname can be accessed through a browser if the bucket is made public. | [optional]
**label** | Option<**String**> | The name of this bucket. | [optional]
**objects** | Option<**i32**> | The number of objects stored in this bucket. | [optional]
**region** | Option<**String**> | The `id` of the [region](https://techdocs.akamai.com/linode-api/reference/get-regions) where this Object Storage bucket lives. | [optional]
**s3_endpoint** | Option<**String**> | The active user's s3 endpoint URL, based on the `endpoint_type` and `region`. | [optional]
**size** | Option<**i32**> | The size of the bucket in bytes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


