# EntityTransfer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | Option<**String**> | When this transfer was created. | [optional]
**entities** | Option<[**models::GetEntityTransfers200ResponseAllOfDataInnerEntities**](get_entity_transfers_200_response_allOf_data_inner_entities.md)> |  | [optional]
**expiry** | Option<**String**> | When this transfer expires. Transfers will automatically expire 24 hours after creation. | [optional]
**is_sender** | Option<**bool**> | __Filterable__ If the requesting account created this transfer. | [optional]
**status** | Option<**String**> | __Filterable__ The status of the transfer request:  `accepted`: The transfer has been accepted by another user and is currently in progress. Transfers can take up to 3 hours to complete. `canceled`: The transfer has been canceled by the sender. `completed`: The transfer has completed successfully. `failed`: The transfer has failed after initiation. `pending`: The transfer is ready to be accepted. `stale`: The transfer has exceeded its expiration date. It can no longer be accepted or canceled. | [optional]
**token** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The token used to identify and accept or cancel this transfer. | [optional]
**updated** | Option<**String**> | When this transfer was last updated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


