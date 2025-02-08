# GetTickets200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attachments** | Option<**Vec<String>**> | __Read-only__ A list of filenames representing attached files associated with this ticket. | [optional][readonly]
**closable** | Option<**bool**> | Whether the ticket can be closed. | [optional]
**closed** | Option<**String**> | __Filterable__, __Read-only__ When this ticket was closed. | [optional][readonly]
**description** | Option<**String**> | __Read-only__ The full details of the issue or question. | [optional][readonly]
**entity** | Option<[**models::GetTickets200ResponseDataInnerEntity**](get_tickets_200_response_data_inner_entity.md)> |  | [optional]
**gravatar_id** | Option<**String**> | __Read-only__ The Gravatar ID of the user who opened this ticket. | [optional][readonly]
**id** | Option<**i32**> | __Read-only__ The ID of the support ticket. | [optional][readonly]
**opened** | Option<**String**> | __Filterable__, __Read-only__ When this ticket was created. | [optional][readonly]
**opened_by** | Option<**String**> | __Read-only__ The user who opened this ticket. | [optional][readonly]
**status** | Option<**String**> | __Read-only__ The current status of this ticket. | [optional][readonly]
**summary** | Option<**String**> | __Read-only__ The summary or title for this ticket. | [optional][readonly]
**updated** | Option<**String**> | __Filterable__, __Read-only__ When this ticket was last updated. | [optional][readonly]
**updated_by** | Option<**String**> | __Read-only__ The user who last updated this ticket. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


