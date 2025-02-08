# GetManagedIssues200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | Option<**String**> | __Read-only__ When this Issue was created. Issues are created in response to issues detected with Managed Services, so this is also when the Issue was detected. | [optional][readonly]
**entity** | Option<[**models::GetManagedIssues200ResponseDataInnerEntity**](get_managed_issues_200_response_data_inner_entity.md)> |  | [optional]
**id** | Option<**i32**> | __Read-only__ This Issue's unique ID. | [optional][readonly]
**services** | Option<**Vec<i32>**> | __Read-only__ An array of Managed Service IDs that were affected by this Issue. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


