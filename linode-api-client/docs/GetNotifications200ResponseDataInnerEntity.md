# GetNotifications200ResponseDataInnerEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique ID of the notification's entity, based on the entity type. Returns `null` for an `account` or `promotion` entity. | [optional]
**label** | Option<**String**> | The current label for this notification's entity.  Returns `null` for the following entity types:  - `entity_transfer` - `promotion` - `region` | [optional]
**r#type** | Option<**String**> | The type of entity this is related to. | [optional]
**url** | Option<**String**> | The URL where you can access the notification's object. The URL is relative to the domain where you retrieved the notification. This value is `null` for the `promotion` entity type. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


