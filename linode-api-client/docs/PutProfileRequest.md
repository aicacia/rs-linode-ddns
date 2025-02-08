# PutProfileRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authentication_type** | Option<**String**> | __Read-only__ This account's Cloud Manager authentication type. Authentication types are chosen through Cloud Manager and authorized when logging into your account. These authentication types are either the user's password (in conjunction with their username), or the name of their identity provider such as GitHub. For example, if a user:  - Has never used Third-Party Authentication, their authentication type will be `password`. - Is using Third-Party Authentication, their authentication type will be the name of their Identity Provider (eg. `github`). - Has used Third-Party Authentication and has since revoked it, their authentication type will be `password`.  __Note__. This functionality may not yet be available in Cloud Manager. See the [Cloud Manager Changelog](https://www.linode.com/docs/products/tools/cloud-manager/release-notes/) for the latest updates. | [optional][readonly]
**authorized_keys** | Option<**Vec<String>**> | The list of SSH Keys authorized to use Lish for your User. This value is ignored if `lish_auth_method` is `disabled`. | [optional]
**email** | Option<**String**> | Your email address.  This address will be used for communication with Linode as necessary. | [optional]
**email_notifications** | Option<**bool**> | If `true`, you will receive email notifications about account activity.  If `false`, you may still receive business-critical communications through email. | [optional]
**ip_whitelist_enabled** | Option<**bool**> | If `true`, logins for your User will only be allowed from whitelisted IPs. This setting is currently deprecated, and cannot be enabled. If you disable this setting, you will not be able to re-enable it. | [optional]
**lish_auth_method** | Option<**String**> | The authentication methods that are allowed when connecting to [the Linode Shell (Lish)](https://www.linode.com/docs/guides/lish/).  - `keys_only` is the most secure if you intend to use Lish. - `disabled` is recommended if you do not intend to use Lish at all. - If this account's Cloud Manager authentication type is set to a Third-Party Authentication method, `password_keys` cannot be used as your Lish authentication method. To view this account's Cloud Manager `authentication_type` field, send a request to the [Get a profile](https://techdocs.akamai.com/linode-api/reference/get-profile) operation. | [optional]
**referrals** | Option<[**models::GetProfile200ResponseReferrals**](get_profile_200_response_referrals.md)> |  | [optional]
**restricted** | Option<**bool**> | If `true`, your User has restrictions on what can be accessed on your Account. To get details on what entities/actions you can access/perform, run [List grants](https://techdocs.akamai.com/linode-api/reference/get-profile-grants). | [optional]
**timezone** | Option<**String**> | The timezone you prefer to see times in. This is not used by the API directly. It is provided for the benefit of clients such as the Linode Cloud Manager and other clients built on the API. All times returned by the API are in UTC. | [optional]
**two_factor_auth** | Option<**bool**> | If `true`, logins from untrusted computers will require Two Factor Authentication.  Run [Create a two factor secret](https://techdocs.akamai.com/linode-api/reference/post-tfa-enable) to enable Two Factor Authentication. | [optional]
**uid** | Option<**i32**> | __Read-only__ Your unique ID in our system. This value will never change, and can safely be used to identify your User. | [optional][readonly]
**username** | Option<**String**> | __Read-only__ Your username, used for logging in to our system. | [optional][readonly]
**verified_phone_number** | Option<**String**> | __Read-only__ The phone number verified for this Profile with the [Verify a phone number](https://techdocs.akamai.com/linode-api/reference/post-profile-phone-number-verify) operation.  `null` if this Profile has no verified phone number. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


