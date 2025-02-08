# PutUserRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | Option<**String**> | The email address for the User. Linode sends emails to this address for account management communications. May be used for other communications as configured. | [optional]
**last_login** | Option<[**models::GetUsers200ResponseDataInnerAllOfLastLogin**](get_users_200_response_data_inner_allOf_last_login.md)> |  | [optional]
**password_created** | Option<**String**> | __Read-only__ The date and time when this User's current password was created.  User passwords are first created during the Account sign-up process, and updated using the [Reset Password](https://login.linode.com/forgot/password) webpage.  `null` if this User has not created a password yet. | [optional][readonly]
**restricted** | Option<**bool**> | If `true`, the User must be granted access to perform actions or access entities on this Account. Run [List a user's grants](https://techdocs.akamai.com/linode-api/reference/get-user-grants) for details on how to configure grants for a restricted User. | [optional]
**ssh_keys** | Option<**Vec<String>**> | __Read-only__ A list of SSH Key labels added by this User.  Users can add keys with the [Add an SSH key](https://techdocs.akamai.com/linode-api/reference/post-add-ssh-key) operation.  These keys are deployed when this User is included in the `authorized_users` field of the following requests:  - [Create a Linode](https://techdocs.akamai.com/linode-api/reference/post-linode-instance) - [Rebuild a Linode](https://techdocs.akamai.com/linode-api/reference/post-rebuild-linode-instance) - [Create a disk](https://techdocs.akamai.com/linode-api/reference/post-add-linode-disk) | [optional][readonly]
**tfa_enabled** | Option<**bool**> | __Read-only__ A boolean value indicating if the User has Two Factor Authentication (TFA) enabled. Run the [Create a two factor secret](https://techdocs.akamai.com/linode-api/reference/post-tfa-enable) operation to enable TFA. | [optional][readonly]
**username** | Option<**String**> | __Filterable__ The User's username. This is used for logging in, and may also be displayed alongside actions the User performs (for example, in Events or public StackScripts). | [optional]
**verified_phone_number** | Option<**String**> | __Read-only__ The phone number verified for this User Profile with the [Verify a phone number](https://techdocs.akamai.com/linode-api/reference/post-profile-phone-number-verify) operation.  `null` if this User Profile has no verified phone number. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


