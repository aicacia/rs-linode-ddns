# TcpNodesStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**down** | Option<**i32**> | __Read-only__ The number of backends considered to be `DOWN` and unhealthy.  These are not in rotation, and not serving requests. | [optional][readonly]
**up** | Option<**i32**> | __Read-only__ The number of backends considered to be `UP` and healthy, and that are serving requests. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


