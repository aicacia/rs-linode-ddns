# \PhoneNumberApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_profile_phone_number**](PhoneNumberApi.md#delete_profile_phone_number) | **DELETE** /{apiVersion}/profile/phone-number | Delete a phone number
[**post_profile_phone_number**](PhoneNumberApi.md#post_profile_phone_number) | **POST** /{apiVersion}/profile/phone-number | Send a phone number verification code
[**post_profile_phone_number_verify**](PhoneNumberApi.md#post_profile_phone_number_verify) | **POST** /{apiVersion}/profile/phone-number/verify | Verify a phone number



## delete_profile_phone_number

> serde_json::Value delete_profile_phone_number(api_version)
Delete a phone number

Delete the verified phone number for the User making this request.  Use this operation to opt out of SMS messages for the requesting User after a phone number has been verified with the [Verify a phone number](https://techdocs.akamai.com/linode-api/reference/post-profile-phone-number-verify) operation.   <<LB>>  ---   - __CLI__.      ```     linode-cli phone delete     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

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


## post_profile_phone_number

> serde_json::Value post_profile_phone_number(api_version, post_profile_phone_number_request)
Send a phone number verification code

Send a one-time verification code via SMS message to the submitted phone number. Providing your phone number helps ensure you can securely access your Account in case other ways to connect are lost. Your phone number is only used to verify your identity by sending an SMS message. Standard carrier messaging fees may apply.  - By accessing this operation you are opting in to receive SMS messages. You can opt out of SMS messages by running the [Delete a phone number](https://techdocs.akamai.com/linode-api/reference/delete-profile-phone-number) operation after your phone number is verified.  - Verification codes are valid for 10 minutes after they are sent.  - Subsequent requests made prior to code expiration result in sending the same code.  Once a verification code is received, verify your phone number with the [Verify a phone number](https://techdocs.akamai.com/linode-api/reference/post-profile-phone-number-verify) operation.   <<LB>>  ---   - __CLI__.      ```     linode-cli phone sms-code-send \\   --iso-code US \\   --phone-number 555-555-5555     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_profile_phone_number_request** | Option<[**PostProfilePhoneNumberRequest**](PostProfilePhoneNumberRequest.md)> | Enter a phone number and country code for verification. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_profile_phone_number_verify

> serde_json::Value post_profile_phone_number_verify(api_version, post_profile_phone_number_verify_request)
Verify a phone number

Verify a phone number by confirming the one-time code received via SMS message after running the [Send a phone number verification code](https://techdocs.akamai.com/linode-api/reference/post-profile-phone-number) operation.  - Verification codes are valid for 10 minutes after they are sent.  - Only the same User that made the verification code request can use that code with this operation.  Once completed, the verified phone number is assigned to the User making the request. To change the verified phone number for a User, first run the [Delete a phone number](https://techdocs.akamai.com/linode-api/reference/delete-profile-phone-number) operation, then begin the verification process again with the [Send a phone number verification code](https://techdocs.akamai.com/linode-api/reference/post-profile-phone-number) operation.   <<LB>>  ---   - __CLI__.      ```     linode-cli phone verify \\   --otp_code 123456     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_profile_phone_number_verify_request** | Option<[**PostProfilePhoneNumberVerifyRequest**](PostProfilePhoneNumberVerifyRequest.md)> | Enter a phone verification code for confirmation. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

