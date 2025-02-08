# Maintenance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entity** | Option<[**models::GetMaintenance200ResponseDataInnerEntity**](get_maintenance_200_response_data_inner_entity.md)> |  | [optional]
**reason** | Option<**String**> | The reason maintenance is being performed. | [optional]
**status** | Option<**String**> | __Filterable__ The maintenance status.  Maintenance progresses in the following sequence: pending, started, then completed. | [optional]
**r#type** | Option<**String**> | __Filterable__ The type of maintenance. | [optional]
**when** | Option<**String**> | __Filterable__ When the maintenance will begin.  [Filterable](https://techdocs.akamai.com/linode-api/reference/filtering-and-sorting) with the following parameters:  - A single value in date-time string format (`%Y-%m-%dT%H:%M:%S`), which returns only matches to that value.  - A dictionary containing pairs of inequality operator string keys (`+or`, `+gt`, `+gte`, `+lt`, `+lte`, or `+neq`) and single date-time string format values (`%Y-%m-%dT%H:%M:%S`). `+or` accepts an array of values that may consist of single date-time strings or dictionaries of inequality operator pairs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


