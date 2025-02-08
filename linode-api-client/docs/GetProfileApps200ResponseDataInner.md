# GetProfileApps200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | Option<**String**> | __Filterable__, __Read-only__ When this app was authorized. | [optional][readonly]
**expiry** | Option<**String**> | __Filterable__, __Read-only__ When the app's access to your account expires. If `null`, the app's access must be revoked manually. | [optional][readonly]
**id** | Option<**i32**> | __Read-only__ This authorization's ID, used for revoking access. | [optional][readonly]
**label** | Option<**String**> | __Filterable__, __Read-only__ The name of the application you've authorized. | [optional][readonly]
**scopes** | Option<**String**> | __Read-only__ The OAuth scopes this app was authorized with.  This defines what parts of your Account the app is allowed to access. | [optional][readonly]
**thumbnail_url** | Option<**String**> | __Read-only__ The URL at which this app's thumbnail may be accessed. | [optional][readonly]
**website** | Option<**String**> | __Read-only__ The website where you can get more information about this app. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


