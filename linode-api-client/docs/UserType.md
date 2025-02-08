# UserType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_type** | Option<**String**> | __Read-only__ If the user belongs to a [parent or child account](https://www.linode.com/docs/guides/parent-child-accounts/) relationship, this defines the user type in the respective account. Possible values include:  - `parent`. This is a user in an Akamai partner account. Akamai partners have a contractural relationship with their end customers, to sell Akamai services. This user can either have full access (a parent account admin user) or limited access. Limited users don't have access to manage child accounts, but they can be granted this access by an admin user.  - `child`. This is an Akamai partner's end customer user, in a child account. A child user can have either full or limited access. Full access lets the user manage other child users and the proxy user in a child account.  - `proxy`. This is a user on a child account that gives parent account users access to that child account. A parent account user with the `child_account_access` grant can [Create a proxy user token](https://techdocs.akamai.com/linode-api/reference/post-child-account-token) from the parent account. The parent user can use this token to run API operations from the child account, as if they were a child user.  - `default`. This applies to all regular, non-parent-child account users. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


