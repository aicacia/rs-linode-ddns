# OauthClient

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | __Read-only__ The OAuth Client ID.  This is used to identify the client, and is a publicly known value (it is not a secret). | [optional][readonly]
**label** | Option<**String**> | __Filterable__ The name of this application.  This will be presented to users when they are asked to grant it access to their Account. | [optional]
**public** | Option<**bool**> | __Filterable__ If this is a public or private OAuth Client.  Public clients have a slightly different authentication workflow than private clients.  See the [OAuth spec](https://oauth.net/2/) for more details. | [optional][default to false]
**redirect_uri** | Option<**String**> | The location a successful log in from [login.linode.com](https://login.linode.com) should be redirected to for this client.  The receiver of this redirect should be ready to accept an OAuth exchange code and finish the OAuth exchange. | [optional]
**secret** | Option<**String**> | __Read-only__ The OAuth Client secret, used in the OAuth exchange.  This is returned as `<REDACTED>` except when an OAuth Client is created or its secret is reset.  This is a secret, and should not be shared or disclosed publicly. | [optional][readonly]
**status** | Option<**String**> | __Read-only__ The status of this application.  `active` by default. | [optional][readonly]
**thumbnail_url** | Option<**String**> | __Read-only__ The URL where this client's thumbnail may be viewed, or `null` if this client does not have a thumbnail set. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


