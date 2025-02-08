# \UsersApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /{apiVersion}/account/users/{username} | Delete a user
[**get_user**](UsersApi.md#get_user) | **GET** /{apiVersion}/account/users/{username} | Get a user
[**get_user_grants**](UsersApi.md#get_user_grants) | **GET** /{apiVersion}/account/users/{username}/grants | List a user's grants
[**get_users**](UsersApi.md#get_users) | **GET** /{apiVersion}/account/users | List users
[**post_user**](UsersApi.md#post_user) | **POST** /{apiVersion}/account/users | Create a user
[**put_user**](UsersApi.md#put_user) | **PUT** /{apiVersion}/account/users/{username} | Update a user
[**put_user_grants**](UsersApi.md#put_user_grants) | **PUT** /{apiVersion}/account/users/{username}/grants | Update a user's grants



## delete_user

> serde_json::Value delete_user(api_version, username)
Delete a user

Deletes a user. The API immediately logs the user out and removes all of its `grants`.  __Note__. This operation can only be accessed by account users with _unrestricted_ access.  __Parent and child accounts__  In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:  - You can't delete a child account parent user (proxy user). The API returns a 403 error if you target a proxy user with this operation.  - A parent account using an unrestricted proxy user can use this operation to delete a child account user.   <<LB>>  ---   - __CLI__.      ```     linode-cli users delete example_user     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**username** | **String** | The username to look up. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> models::GetUser200Response get_user(api_version, username)
Get a user

Returns information about a single user on your account.  __Note__. This operation can only be accessed by account users with _unrestricted_ access.   <<LB>>  ---   - __CLI__.      ```     linode-cli users view example_user     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**username** | **String** | The username to look up. | [required] |

### Return type

[**models::GetUser200Response**](get_user_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_grants

> models::GetUserGrants200Response get_user_grants(api_version, username)
List a user's grants

Returns the full grants structure for an account username you specify. This includes all entities on the account, and the level of access this user has to each of them.  This doesn't apply to the account owner or the current authenticated user. You can run the [List grants](https://techdocs.akamai.com/linode-api/reference/get-profile-grants) operation to view those grants. However, this doesn't show the entities that they _don't_ have access to.  __Note__. This operation can only be accessed by account users with _unrestricted_ access.   <<LB>>  ---   - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**username** | **String** | The username to look up. | [required] |

### Return type

[**models::GetUserGrants200Response**](get_user_grants_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users

> models::GetUsers200Response get_users(api_version, page, page_size)
List users

Returns a paginated list of all users on your account.  __Note__. This operation can only be accessed by account users with _unrestricted_ access.  A user can access all or part of an account based on their access status and grants:  - __Unrestricted access__. These users can access everything on an account.  - __Restricted access__. These users can only access entities or perform actions they've been given specific grants to.   <<LB>>  ---   - __CLI__.      ```     linode-cli users list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetUsers200Response**](get_users_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_user

> models::PostUser200Response post_user(api_version, post_user_request)
Create a user

Creates a user on your account. You determine the new user's account access by setting it to restricted or unrestricted and by defining its grants. After completion, the API sends a confirmation message containing password creation and login instructions to the user's `email` address.  __Note__. This operation can only be accessed by account users with _unrestricted_ access.  __Parent and child accounts__  In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:  - A parent account user can create new parent account users.  - A child account can [update](https://techdocs.akamai.com/linode-api/reference/put-user) the child account parent user (proxy user) to `unrestricted`. This gives the proxy user access to create new child account users.  - A child account user can create new child account users.  - You can't create a proxy user. The proxy user in a child account is predefined when you initially provision the parent-child relationship.   <<LB>>  ---   - __CLI__.      ```     linode-cli users create \\   --username example_user \\   --email example_user@linode.com \\   --restricted true     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_user_request** | Option<[**PostUserRequest**](PostUserRequest.md)> | Information about the User to create. |  |

### Return type

[**models::PostUser200Response**](post_user_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_user

> models::PutUser200Response put_user(api_version, username, put_user_request)
Update a user

Update information about a user on your account, including its restricted status. When setting a user to `restricted`, the API sets no grants for it. You need to set grants so that user can access things on the account.  __Note__. This operation can only be accessed by account users with _unrestricted_ access.  __Parent and child accounts__  In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:  - You can't edit the `username` or `email` values for the child account parent user (proxy user). These are predefined for the proxy user when you initially provision the parent-child relationship. Only a proxy user's `restricted` status can be modified. This can only be done by an unrestricted child account user.  - A parent account using an unrestricted proxy user in a child account can modify the `username`, `email`, and `restricted` status for an existing child account user.  - A restricted account user--parent or child--can't change their user to `unrestricted`.   <<LB>>  ---   - __CLI__.      ```     linode-cli users update example_user \\   --username example_user \\   --email example@linode.com \\   --restricted true     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**username** | **String** | The username to look up. | [required] |
**put_user_request** | Option<[**PutUserRequest**](PutUserRequest.md)> | The information to update. |  |

### Return type

[**models::PutUser200Response**](put_user_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_user_grants

> models::GetUserGrants200Response put_user_grants(api_version, username, get_user_grants200_response)
Update a user's grants

Update the grants a user has. This can be used to give a user access to new entities or actions, or take access away.  You don't need to include the grant for every entity on the account in this request. Any that are not included remain unchanged.  __Note__. This operation can only be accessed by account users with _unrestricted_ access.  __Parent and child accounts__  In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:  - No child account user can modify the `account_access` grant for the child account parent user (proxy user).  - An unrestricted child account user can configure all other grants for the proxy user, via `global` object.  - An unrestricted child account user can enable the `account_access` grant for other child account users. However, enabled child users are still subject to child user restrictions--they can't perform write operations for any billing or account information.   <<LB>>  ---   - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**username** | **String** | The username to look up. | [required] |
**get_user_grants200_response** | [**GetUserGrants200Response**](GetUserGrants200Response.md) | The grants to update. Omitted grants will be left unchanged. | [required] |

### Return type

[**models::GetUserGrants200Response**](get_user_grants_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

