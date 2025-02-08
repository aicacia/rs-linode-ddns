# GetIps200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**page** | Option<**i32**> | __Read-only__ The current [page](https://techdocs.akamai.com/linode-api/reference/pagination). | [optional][readonly]
**pages** | Option<**i32**> | __Read-only__ The total number of [pages](https://techdocs.akamai.com/linode-api/reference/pagination). | [optional][readonly]
**results** | Option<**i32**> | __Read-only__ The total number of results. | [optional][readonly]
**data** | Option<[**Vec<models::GetIps200ResponseAllOfDataInner>**](get_ips_200_response_allOf_data_inner.md)> | IP addresses that exist in Linode's system, either IPv4 or IPv6, specific to the response for the [List IP addresses](https://techdocs.akamai.com/linode-api/reference/get-ips) operation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


