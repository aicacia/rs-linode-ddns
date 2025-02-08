# PostTagRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domains** | Option<**Vec<i32>**> | A list of Domain IDs to apply the new Tag to.  You must be allowed to `read_write` all of the requested Domains, or the Tag will not be created and an error will be returned. | [optional]
**label** | **String** | The new Tag. | 
**linodes** | Option<**Vec<i32>**> | A list of Linode IDs to apply the new Tag to.  You must be allowed to `read_write` all of the requested Linodes, or the Tag will not be created and an error will be returned. | [optional]
**nodebalancers** | Option<**Vec<i32>**> | A list of NodeBalancer IDs to apply the new Tag to. You must be allowed to `read_write` all of the requested NodeBalancers, or the Tag will not be created and an error will be returned. | [optional]
**volumes** | Option<**Vec<i32>**> | A list of Volume IDs to apply the new Tag to.  You must be allowed to `read_write` all of the requested Volumes, or the Tag will not be created and an error will be returned. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


