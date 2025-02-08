# FilterAndSortCriteria

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**plus_and** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | All conditions need to be true. | [optional]
**plus_contains** | Option<**String**> | The provided string needs to be in the value. | [optional]
**plus_gt** | Option<**f64**> | The value needs to be greater than the provided number. | [optional]
**plus_gte** | Option<**f64**> | The value needs to be greater than or equal to the provided number. | [optional]
**plus_lt** | Option<**f64**> | The value needs to be less than the provided number. | [optional]
**plus_lte** | Option<**f64**> | The value needs to be less than or equal to the provided number. | [optional]
**plus_neq** | Option<**String**> | The provided string is left out of the results. | [optional]
**plus_or** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | At least one condition needs to be true. | [optional]
**plus_order** | Option<**String**> | Sort in ascending (`asc`) or descending (`desc`) order. This defaults to `asc`. Requires `+order_by`. | [optional][default to Asc]
**plus_order_by** | Option<**String**> | Order results based on the provided attribute. The attribute needs to be filterable. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


