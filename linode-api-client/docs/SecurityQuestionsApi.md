# \SecurityQuestionsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_security_questions**](SecurityQuestionsApi.md#get_security_questions) | **GET** /{apiVersion}/profile/security-questions | List security questions
[**post_security_questions**](SecurityQuestionsApi.md#post_security_questions) | **POST** /{apiVersion}/profile/security-questions | Answer security questions



## get_security_questions

> models::GetSecurityQuestions200Response get_security_questions(api_version)
List security questions

Returns a collection of security questions and their responses, if any, for your User Profile.   <<LB>>  ---   - __CLI__.      ```     linode-cli security-questions list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetSecurityQuestions200Response**](get_security_questions_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_security_questions

> models::PostSecurityQuestionsRequest post_security_questions(api_version, post_security_questions_request)
Answer security questions

Adds security question responses for your User.  Requires exactly three unique questions.  Previous responses are overwritten if answered or reset to `null` if unanswered.  __Note__. Security questions must be answered for your User prior to accessing the [Create a two factor secret](https://techdocs.akamai.com/linode-api/reference/post-tfa-enable) operation.   <<LB>>  ---   - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_security_questions_request** | Option<[**PostSecurityQuestionsRequest**](PostSecurityQuestionsRequest.md)> | Answer Security Questions. |  |

### Return type

[**models::PostSecurityQuestionsRequest**](post_security_questions_request.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

