# PutPersonalAccessTokenRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | Option<**String**> | __Filterable__, __Read-only__ The date and time this token was created. | [optional][readonly]
**expiry** | Option<**String**> | __Read-only__ When this token will expire.  Personal Access Tokens cannot be renewed, so after this time the token will be completely unusable and a new token will need to be generated.  Tokens may be created with `null` as their expiry and will never expire unless revoked. | [optional][readonly]
**id** | Option<**i32**> | __Read-only__ This token's unique ID, which can be used to revoke it. | [optional][readonly]
**label** | Option<**String**> | __Filterable__ This token's label.  This is for display purposes only, but can be used to more easily track what you're using each token for. | [optional]
**scopes** | Option<**String**> | __Read-only__ The scopes this token was created with. These define what parts of the Account the token can be used to access. Many command-line tools, such as the [Linode CLI](https://github.com/linode/linode-cli), require tokens with access to `*`. Tokens with more restrictive scopes are generally more secure. | [optional][readonly]
**token** | Option<**String**> | __Read-only__ The token used to access the API.  When the token is created, the full token is returned here.  Otherwise, only the first 16 characters are returned. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


