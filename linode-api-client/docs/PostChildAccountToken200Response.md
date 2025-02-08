# PostChildAccountToken200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | Option<**String**> | __Filterable__, __Read-only__ The date and time this token was created. | [optional][readonly]
**expiry** | Option<**String**> | __Read-only__ When this token expires. This is default set to 15 minutes from the time of creation. Proxy user tokens can't be renewed. After this time, Akamai revokes the token and you need to generate a new one. | [optional][readonly]
**id** | Option<**i32**> | __Read-only__ The proxy user token's unique ID, which can be used to revoke it. | [optional][readonly]
**label** | Option<**String**> | __Filterable__ The name of the token. The API automatically sets this to `<username>_<uid>_<time>`. It's composed of the `username` for your parent account user, the unique `uid` Akamai assigned to identify your user, and the `time` the API generated the token. This is for display purposes only, but you can use it to help track how you're using each proxy user token. | [optional]
**scopes** | Option<**String**> | __Read-only__ The scopes this token was created with. Defaults to `*`. Proxy user tokens automatically inherit all the permissions of the proxy user. | [optional][readonly]
**token** | Option<**String**> | __Read-only__ The proxy user token that can be used to access the API and CLI. After you [create](https://techdocs.akamai.com/linode-api/reference/post-child-account-token) a token, you can see the full token in the response. All other operations that contain this token only show the first 16 characters in their response. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


