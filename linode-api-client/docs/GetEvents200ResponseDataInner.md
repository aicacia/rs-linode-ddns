# GetEvents200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | Option<**String**> | __Filterable__, __Read-only__ The action that caused this Event. New actions may be added in the future. | [optional][readonly]
**created** | Option<**String**> | __Filterable__, __Read-only__ When this Event was created. | [optional][readonly]
**duration** | Option<**f64**> | __Read-only__ The total duration in seconds that it takes for the Event to complete. | [optional][readonly]
**entity** | Option<[**models::GetEvents200ResponseDataInnerEntity**](get_events_200_response_data_inner_entity.md)> |  | [optional]
**id** | Option<**i32**> | __Filterable__, __Read-only__ The unique ID of this Event. | [optional][readonly]
**message** | Option<**String**> | Provides additional information about the event. Additional information may include, but is not limited to, a more detailed representation of events which can help diagnose non-obvious failures. | [optional]
**percent_complete** | Option<**i32**> | __Read-only__ A percentage estimating the amount of time remaining for an Event. Returns `null` for notification events. | [optional][readonly]
**rate** | Option<**String**> | __Read-only__ The rate of completion of the Event. Only some Events will return rate; for example, migration and resize Events. | [optional][readonly]
**read** | Option<**bool**> | __Filterable__, __Read-only__ If this Event has been read. | [optional][readonly]
**secondary_entity** | Option<[**models::GetEvents200ResponseDataInnerSecondaryEntity**](get_events_200_response_data_inner_secondary_entity.md)> |  | [optional]
**seen** | Option<**bool**> | __Read-only__ If this Event has been seen. | [optional][readonly]
**status** | Option<**String**> | __Read-only__ The current status of this Event. | [optional][readonly]
**time_remaining** | Option<**String**> | __Read-only__ The estimated time remaining until the completion of this Event. This value is only returned for some in-progress migration events. For all other in-progress events, the `percent_complete` attribute will indicate about how much more work is to be done. | [optional][readonly]
**username** | Option<**String**> | __Read-only__ The username of the User who caused the Event. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


