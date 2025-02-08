# PostPersonalAccessTokenRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expiry** | Option<**String**> | When this token should be valid until.  If omitted, the new token will be valid until it is manually revoked. | [optional]
**label** | Option<**String**> | __Filterable__ This token's label.  This is for display purposes only, but can be used to more easily track what you're using each token for. | [optional]
**scopes** | Option<**String**> | The access [scopes](https://techdocs.akamai.com/linode-api/reference/get-started#oauth-reference) to grant to the created token. These cannot be changed after creation, and may not exceed the scopes of the acting token.  If omitted or entered with a wildcard character (`*`), the new token will have the same scopes as the acting token.  Multiple scopes are separated by a space character (` `).  For example, `linodes:read_write account:read_only`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


