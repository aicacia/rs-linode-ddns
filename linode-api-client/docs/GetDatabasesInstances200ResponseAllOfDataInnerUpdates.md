# GetDatabasesInstances200ResponseAllOfDataInnerUpdates

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**day_of_week** | Option<**i32**> | The numeric reference for the day of the week to perform maintenance. `1` is Monday, `2` is Tuesday, through to `7` which is Sunday. | [optional]
**duration** | Option<**i32**> | The maximum maintenance window time in hours. | [optional]
**frequency** | Option<**String**> | How frequently maintenance occurs. Currently can only be `weekly`. | [optional][default to Weekly]
**hour_of_day** | Option<**i32**> | The hour to begin maintenance based in UTC time. | [optional]
**pending** | Option<[**Vec<models::GetDatabasesInstances200ResponseAllOfDataInnerUpdatesPendingInner>**](get_databases_instances_200_response_allOf_data_inner_updates_pending_inner.md)> | __Read-only__ An array of pending updates. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


