# GetLongviewClients200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_key** | Option<**String**> | __Read-only__ The API key for this Client, used when configuring the Longview Client application on your Linode.  Returns as `[REDACTED]` if you do not have read-write access to this client. | [optional][readonly]
**apps** | Option<[**models::GetLongviewClients200ResponseDataInnerApps**](get_longview_clients_200_response_data_inner_apps.md)> |  | [optional]
**created** | Option<**String**> | __Read-only__ When this Longview Client was created. | [optional][readonly]
**id** | Option<**i32**> | __Read-only__ This Client's unique ID. | [optional][readonly]
**install_code** | Option<**String**> | __Read-only__ The install code for this Client, used when configuring the Longview Client application on your Linode.  Returns as `[REDACTED]` if you do not have read-write access to this client. | [optional][readonly]
**label** | Option<**String**> | __Filterable__ This Client's unique label. This is for display purposes only. | [optional]
**updated** | Option<**String**> | __Read-only__ When this Longview Client was last updated. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


