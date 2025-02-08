# \OAuthClientsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_client**](OAuthClientsApi.md#delete_client) | **DELETE** /{apiVersion}/account/oauth-clients/{clientId} | Delete an OAuth client
[**get_client**](OAuthClientsApi.md#get_client) | **GET** /{apiVersion}/account/oauth-clients/{clientId} | Get an OAuth client
[**get_clients**](OAuthClientsApi.md#get_clients) | **GET** /{apiVersion}/account/oauth-clients | List OAuth clients
[**post_client**](OAuthClientsApi.md#post_client) | **POST** /{apiVersion}/account/oauth-clients | Create an OAuth client
[**post_reset_client_secret**](OAuthClientsApi.md#post_reset_client_secret) | **POST** /{apiVersion}/account/oauth-clients/{clientId}/reset-secret | Reset an OAuth client secret
[**put_client**](OAuthClientsApi.md#put_client) | **PUT** /{apiVersion}/account/oauth-clients/{clientId} | Update an OAuth client



## delete_client

> serde_json::Value delete_client(api_version, client_id)
Delete an OAuth client

Deletes an OAuth Client registered with Linode. The Client ID and Client secret will no longer be accepted by [login.linode.com](https://login.linode.com), and all tokens issued to this client will be invalidated (meaning that if your application was using a token, it will no longer work).   <<LB>>  ---   - __CLI__.      ```     linode-cli account client-delete \\   edc6790ea9db4d224c5c     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**client_id** | **String** | The OAuth Client ID to look up. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client

> models::GetClients200ResponseDataInner get_client(api_version, client_id)
Get an OAuth client

Returns information about a single OAuth client.   <<LB>>  ---   - __CLI__.      ```     linode-cli account client-view \\   edc6790ea9db4d224c5c     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**client_id** | **String** | The OAuth Client ID to look up. | [required] |

### Return type

[**models::GetClients200ResponseDataInner**](get_clients_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_clients

> models::GetClients200Response get_clients(api_version, page, page_size)
List OAuth clients

Returns a paginated list of OAuth Clients registered to your Account.  OAuth Clients allow users to log into applications you write or host using their Linode Account, and may allow them to grant some level of access to their Linodes or other entities to your application.   <<LB>>  ---   - __CLI__.      ```     linode-cli account clients-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetClients200Response**](get_clients_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_client

> models::GetClients200ResponseDataInner post_client(api_version, post_client_request)
Create an OAuth client

Creates an OAuth Client, which can be used to allow users (using their Linode account) to log in to your own application, and optionally grant your application some amount of access to their Linodes or other entities.   <<LB>>  ---   - __CLI__.      ```     linode-cli account client-create \\   --label Test_Client_1 \\   --redirect_uri https://example.org/callback     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_client_request** | Option<[**PostClientRequest**](PostClientRequest.md)> | Information about the OAuth Client to create. |  |

### Return type

[**models::GetClients200ResponseDataInner**](get_clients_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_reset_client_secret

> models::GetClients200ResponseDataInner post_reset_client_secret(api_version, client_id)
Reset an OAuth client secret

Resets the OAuth Client secret for a client you own, and returns the OAuth Client with the plaintext secret. This secret is not supposed to be publicly known or disclosed anywhere. This can be used to generate a new secret in case the one you have has been leaked, or to get a new secret if you lost the original. The old secret is expired immediately, and logins to your client with the old secret will fail.   <<LB>>  ---   - __CLI__.      ```     linode-cli account client-reset-secret \\   edc6790ea9db4d224c5c     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**client_id** | **String** | The OAuth Client ID to look up. | [required] |

### Return type

[**models::GetClients200ResponseDataInner**](get_clients_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_client

> models::GetClients200ResponseDataInner put_client(api_version, client_id, put_client_request)
Update an OAuth client

Update information about an OAuth Client on your Account. This can be especially useful to update the `redirect_uri` of your client in the event that the callback URL changed in your application.   <<LB>>  ---   - __CLI__.      ```     linode-cli account client-update \\   edc6790ea9db4d224c5c \\   --label Test_Client_1     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**client_id** | **String** | The OAuth Client ID to look up. | [required] |
**put_client_request** | Option<[**PutClientRequest**](PutClientRequest.md)> | The fields to update. |  |

### Return type

[**models::GetClients200ResponseDataInner**](get_clients_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

