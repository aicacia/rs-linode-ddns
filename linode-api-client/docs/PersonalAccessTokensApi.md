# \PersonalAccessTokensApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_personal_access_token**](PersonalAccessTokensApi.md#delete_personal_access_token) | **DELETE** /{apiVersion}/profile/tokens/{tokenId} | Revoke a personal access token
[**get_personal_access_token**](PersonalAccessTokensApi.md#get_personal_access_token) | **GET** /{apiVersion}/profile/tokens/{tokenId} | Get a personal access token
[**get_personal_access_tokens**](PersonalAccessTokensApi.md#get_personal_access_tokens) | **GET** /{apiVersion}/profile/tokens | List personal access tokens
[**post_personal_access_token**](PersonalAccessTokensApi.md#post_personal_access_token) | **POST** /{apiVersion}/profile/tokens | Create a personal access token
[**put_personal_access_token**](PersonalAccessTokensApi.md#put_personal_access_token) | **PUT** /{apiVersion}/profile/tokens/{tokenId} | Update a personal access token



## delete_personal_access_token

> serde_json::Value delete_personal_access_token(api_version, token_id)
Revoke a personal access token

Revokes a Personal Access Token. The token will be invalidated immediately, and requests using that token will fail with a 401. It is possible to revoke access to the token making the request to revoke a token, but keep in mind that doing so could lose you access to the api and require you to create a new token through some other means.   <<LB>>  ---   - __CLI__.      ```     linode-cli profile token-delete 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**token_id** | **i32** | The ID of the token to access. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_personal_access_token

> models::GetPersonalAccessTokens200ResponseDataInner get_personal_access_token(api_version, token_id)
Get a personal access token

Returns a single Personal Access Token.   <<LB>>  ---   - __CLI__.      ```     linode-cli profile token-view 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**token_id** | **i32** | The ID of the token to access. | [required] |

### Return type

[**models::GetPersonalAccessTokens200ResponseDataInner**](get_personal_access_tokens_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_personal_access_tokens

> models::GetPersonalAccessTokens200Response get_personal_access_tokens(api_version)
List personal access tokens

Returns a paginated list of Personal Access Tokens currently active for your User.   <<LB>>  ---   - __CLI__.      ```     linode-cli profile tokens-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetPersonalAccessTokens200Response**](get_personal_access_tokens_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_personal_access_token

> models::GetPersonalAccessTokens200ResponseDataInner post_personal_access_token(api_version, post_personal_access_token_request)
Create a personal access token

Creates a Personal Access Token for your User. The raw token will be returned in the response, but will never be returned again afterward so be sure to take note of it. You may create a token with _at most_ the scopes of your current token. The created token will be able to access your Account until the given expiry, or until it is revoked. __Parent and child accounts__ In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:  - If you're using a child account parent user (proxy user), you can't create this form of token. The only token available to a proxy user is one that lets you run operations in a child account. These are created with the [Create a proxy user token](https://techdocs.akamai.com/linode-api/reference/post-child-account-token) operation.   <<LB>>  ---   - __CLI__.      ```     linode-cli profile token-create \\   --scopes '*' \\   --expiry '2018-01-01T13:46:32' \\   --label linode-cli     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_personal_access_token_request** | [**PostPersonalAccessTokenRequest**](PostPersonalAccessTokenRequest.md) | Information about the requested token. | [required] |

### Return type

[**models::GetPersonalAccessTokens200ResponseDataInner**](get_personal_access_tokens_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_personal_access_token

> models::GetPersonalAccessTokens200ResponseDataInner put_personal_access_token(api_version, token_id, put_personal_access_token_request)
Update a personal access token

Updates a Personal Access Token.   <<LB>>  ---   - __CLI__.      ```     linode-cli profile token-update 123 \\   --label linode-cli     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**token_id** | **i32** | The ID of the token to access. | [required] |
**put_personal_access_token_request** | [**PutPersonalAccessTokenRequest**](PutPersonalAccessTokenRequest.md) | The fields to update. | [required] |

### Return type

[**models::GetPersonalAccessTokens200ResponseDataInner**](get_personal_access_tokens_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

