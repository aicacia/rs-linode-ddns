# \ChildAccountsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_child_account**](ChildAccountsApi.md#get_child_account) | **GET** /{apiVersion}/account/child-accounts/{euuid} | Get a child account
[**get_child_accounts**](ChildAccountsApi.md#get_child_accounts) | **GET** /{apiVersion}/account/child-accounts | List child accounts
[**post_child_account_token**](ChildAccountsApi.md#post_child_account_token) | **POST** /{apiVersion}/account/child-accounts/{euuid}/token | Create a proxy user token



## get_child_account

> models::GetChildAccounts200ResponseDataInner get_child_account(api_version, euuid)
Get a child account

View a specific child account based on its `euuid`. See [Parent and Child Accounts for Akamai Partners](https://www.linode.com/docs/guides/parent-child-accounts/) for details on these accounts.  __Note__. This operation can only be accessed by an unrestricted user, or restricted user with the `child_account_access` grant.   <<LB>>  ---   - __CLI__.      ```     linode-cli child-account view A1BC2DEF-34GH-567I-J890KLMN12O34P56     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     child_account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**euuid** | **String** | The child account to look up. You can run the [List child accounts](https://techdocs.akamai.com/linode-api/reference/get-child-accounts) operation to find the applicable account and store its `euuid`. | [required] |

### Return type

[**models::GetChildAccounts200ResponseDataInner**](get_child_accounts_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_child_accounts

> models::GetChildAccounts200Response get_child_accounts(api_version, page, page_size)
List child accounts

Returns a paginated list of basic information for the child accounts that exist for your parent account. See [Parent and Child Accounts for Akamai Partners](https://www.linode.com/docs/guides/parent-child-accounts/) for details on these accounts.  __Note__. This operation can only be accessed by an unrestricted parent user, or restricted parent user with the `child_account_access` grant.   <<LB>>  ---   - __CLI__.      ```     linode-cli child-account list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     child_account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetChildAccounts200Response**](get_child_accounts_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_child_account_token

> models::PostChildAccountToken200Response post_child_account_token(api_version, euuid)
Create a proxy user token

Create a short-lived bearer token for a parent user on a child account, using the `euuid` of that child account. In the context of the API, a parent user on a child account is referred to as a \"proxy user.\" When Akamai provisions your parent-child account environment, a proxy user is automatically set in the child account. It follows a specific naming convention:      <Parent account `company` name>_<SHA256 hash of parent `company` name and child account `euuid`>  __Note__. The variables above use only the first 15 and 16 characters of these values, respectively.  The token lets a parent account run API operations through the proxy user, as if they are a child user in the child account.  These points apply to the use of this operation:  - To create a token, a parent account user needs the `child_account_access` grant. This lets them use the proxy user on the child account. You can run [List a user's grants](https://techdocs.akamai.com/linode-api/reference/get-user-grants) on a parent account user to check its `child_account_access` setting. To add this access, you can [update](https://techdocs.akamai.com/linode-api/reference/put-user-grants) the parent account user.  - The created token inherits the permissions of the proxy user. It will never have less.  - The API returns the raw token in the response. You can't get it again, so be sure to store it.  Example workflow:  1. [List child accounts](https://techdocs.akamai.com/linode-api/reference/get-child-accounts) and store the `euuid` for the applicable one. 2. Run this operation and store the `token` that's created for the proxy user. 3. As a parent account user with access to the proxy user in the child account, use this `token` to authenticate API operations, as if you were a child user.   <<LB>>  ---   - __CLI__.      ```     linode-cli child-account create A1BC2DEF-34GH-567I-J890KLMN12O34P56     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     child_account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**euuid** | **String** | The child account to look up. You can run the [List child accounts](https://techdocs.akamai.com/linode-api/reference/get-child-accounts) operation to find the applicable account and store its `euuid`. | [required] |

### Return type

[**models::PostChildAccountToken200Response**](post_child_account_token_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

