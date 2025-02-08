# GetDatabasesTypes200ResponseAllOfDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**class** | Option<**String**> | The compute class category. | [optional]
**disk** | Option<**i32**> | The amount of disk space set aside for Databases of this plan type. The value is represented in megabytes. | [optional]
**engines** | Option<[**models::GetDatabasesTypes200ResponseAllOfDataInnerEngines**](get_databases_types_200_response_allOf_data_inner_engines.md)> |  | [optional]
**id** | Option<**String**> | __Read-only__ The ID representing the Managed Database node plan type. | [optional][readonly]
**label** | Option<**String**> | __Read-only__ A human-readable string that describes each plan type. For display purposes only. | [optional][readonly]
**memory** | Option<**i32**> | The amount of RAM allocated to Database created of this plan type. The value is represented in megabytes. | [optional]
**vcpus** | Option<**i32**> | The number of CPUs allocated to databases of this plan type. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


