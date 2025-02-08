# GetObjectStorageEndpoints200ResponseAllOfDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**endpoint_type** | Option<**String**> | __Read-only__ The type of `s3_endpoint` available to the active `user` in this `region`. See [Endpoint types](https://techdocs.akamai.com/cloud-computing/docs/object-storage#endpoint-type) for more information. | [optional][readonly]
**region** | Option<**String**> | __Read-only__ The Akamai cloud computing region, represented by its slug value. Run the [List regions](https://techdocs.akamai.com/linode-api/reference/get-regions) operation to view all regions and their slugs. | [optional][readonly]
**s3_endpoint** | Option<**String**> | __Read-only__ Your s3 endpoint URL, based on the `endpoint_type` and `region`. Displayed as `null` if you haven't assigned an endpoint for your user. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


