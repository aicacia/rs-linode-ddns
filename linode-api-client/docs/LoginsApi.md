# \LoginsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account_login**](LoginsApi.md#get_account_login) | **GET** /{apiVersion}/account/logins/{loginId} | Get an account login
[**get_account_logins**](LoginsApi.md#get_account_logins) | **GET** /{apiVersion}/account/logins | List user logins
[**get_profile_login**](LoginsApi.md#get_profile_login) | **GET** /{apiVersion}/profile/logins/{loginId} | Get a profile's login
[**get_profile_logins**](LoginsApi.md#get_profile_logins) | **GET** /{apiVersion}/profile/logins | List logins



## get_account_login

> models::GetAccountLogins200ResponseDataInner get_account_login(api_version, login_id)
Get an account login

Returns a Login object that displays information about a successful login. The logins that can be viewed can be for any user on the account, and are not limited to only the logins of the user that is accessing this API endpoint. This operation can only be accessed by the unrestricted users of the account.   <<LB>>  ---   - __CLI__.      ```     linode-cli account login-view 1234     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**login_id** | **i32** | The ID of the login object to access. | [required] |

### Return type

[**models::GetAccountLogins200ResponseDataInner**](get_account_logins_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_logins

> models::GetAccountLogins200Response get_account_logins(api_version)
List user logins

Returns a collection of successful logins for all users on the account during the last 90 days. This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli account logins-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetAccountLogins200Response**](get_account_logins_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profile_login

> models::GetAccountLogins200ResponseDataInner get_profile_login(api_version, login_id)
Get a profile's login

Returns a login object displaying information about a successful account login from this user.   <<LB>>  ---   - __CLI__.      ```     linode-cli profile login-view 1234     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**login_id** | **i32** | The ID of the login object to access. | [required] |

### Return type

[**models::GetAccountLogins200ResponseDataInner**](get_account_logins_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profile_logins

> models::GetProfileLogins200Response get_profile_logins(api_version)
List logins

Returns a collection of successful account logins from this user during the last 90 days.   <<LB>>  ---   - __CLI__.      ```     linode-cli profile logins-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetProfileLogins200Response**](get_profile_logins_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

