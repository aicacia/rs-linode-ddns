# \TwoFactorAuthenticationApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_tfa_confirm**](TwoFactorAuthenticationApi.md#post_tfa_confirm) | **POST** /{apiVersion}/profile/tfa-enable-confirm | Enable two factor authentication
[**post_tfa_disable**](TwoFactorAuthenticationApi.md#post_tfa_disable) | **POST** /{apiVersion}/profile/tfa-disable | Disable two factor authentication
[**post_tfa_enable**](TwoFactorAuthenticationApi.md#post_tfa_enable) | **POST** /{apiVersion}/profile/tfa-enable | Create a two factor secret



## post_tfa_confirm

> models::PostTfaConfirm200Response post_tfa_confirm(api_version, post_tfa_confirm_request)
Enable two factor authentication

Confirms that you can successfully generate Two Factor codes and enables TFA on your Account. Once this is complete, login attempts from untrusted computers will be required to provide a Two Factor code before they are successful.   <<LB>>  ---   - __CLI__.      ```     linode-cli profile tfa-confirm \\   --tfa_code 213456     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_tfa_confirm_request** | [**PostTfaConfirmRequest**](PostTfaConfirmRequest.md) | The Two Factor code you generated with your Two Factor secret. | [required] |

### Return type

[**models::PostTfaConfirm200Response**](post_tfa_confirm_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_tfa_disable

> serde_json::Value post_tfa_disable(api_version)
Disable two factor authentication

Disables Two Factor Authentication for your User. Once successful, login attempts from untrusted computers will only require a password before being successful. This is less secure, and is discouraged.   <<LB>>  ---   - __CLI__.      ```     linode-cli profile tfa-disable     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_tfa_enable

> models::PostTfaEnable200Response post_tfa_enable(api_version)
Create a two factor secret

Generates a Two Factor secret for your User. To enable TFA for your User, enter the secret obtained from this operation with the [Enable two factor authentication](https://techdocs.akamai.com/linode-api/reference/post-tfa-confirm) operation. Once enabled, logins from untrusted computers are required to provide a TFA code before they are successful.  Run the [Answer security questions](https://techdocs.akamai.com/linode-api/reference/post-security-questions) operation.   <<LB>>  ---   - __CLI__.      ```     linode-cli profile tfa-enable     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::PostTfaEnable200Response**](post_tfa_enable_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

