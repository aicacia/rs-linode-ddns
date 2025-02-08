# GetNotifications200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**body** | Option<**String**> | __Read-only__ A full description of this notification, in markdown format. Not all notifications include a `body`. | [optional][readonly]
**entity** | Option<[**models::GetNotifications200ResponseDataInnerEntity**](get_notifications_200_response_data_inner_entity.md)> |  | [optional]
**label** | Option<**String**> | __Read-only__ A short description of this notification. | [optional][readonly]
**message** | Option<**String**> | __Read-only__ A human-readable description of the notification. | [optional][readonly]
**severity** | Option<**String**> | __Read-only__ The severity of this notification. This field determines how prominently the notification is displayed and the color of the display text. | [optional][readonly]
**r#type** | Option<**String**> | __Read-only__ The type of notification. | [optional][readonly]
**until** | Option<**String**> | __Read-only__ If this notification has a duration, this is when the event or action will complete. For example, if there's scheduled maintenance for one of our systems, `until` represents the end of the maintenance window. | [optional][readonly]
**when** | Option<**String**> | __Read-only__ If this notification is for an event in the future, this specifies when the action occurs. For example, if a compute instance needs to migrate in response to a security advisory, this field sets the approximate time the compute instance will be taken offline for migration. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


