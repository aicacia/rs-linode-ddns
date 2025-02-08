/*
 * Akamai: Linode API
 *
 * Add a Cloud Computing instance so you can build, release, and scale applications faster with virtual machines. 
 *
 * The version of the OpenAPI document: 4.193.0
 * Contact: jperez@linode.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// User : A User on your Account. Unrestricted users can log in and access information about your Account, while restricted users may only access entities or perform actions they've been granted access to.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// The email address for the User. Linode sends emails to this address for account management communications. May be used for other communications as configured.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "last_login", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_login: Option<Option<models::GetUsers200ResponseDataInnerAllOfLastLogin>>,
    /// __Read-only__ The date and time when this User's current password was created.  User passwords are first created during the Account sign-up process, and updated using the [Reset Password](https://login.linode.com/forgot/password) webpage.  `null` if this User has not created a password yet.
    #[serde(rename = "password_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub password_created: Option<Option<String>>,
    /// If `true`, the User must be granted access to perform actions or access entities on this Account. Run [List a user's grants](https://techdocs.akamai.com/linode-api/reference/get-user-grants) for details on how to configure grants for a restricted User.
    #[serde(rename = "restricted", skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// __Read-only__ A list of SSH Key labels added by this User.  Users can add keys with the [Add an SSH key](https://techdocs.akamai.com/linode-api/reference/post-add-ssh-key) operation.  These keys are deployed when this User is included in the `authorized_users` field of the following requests:  - [Create a Linode](https://techdocs.akamai.com/linode-api/reference/post-linode-instance) - [Rebuild a Linode](https://techdocs.akamai.com/linode-api/reference/post-rebuild-linode-instance) - [Create a disk](https://techdocs.akamai.com/linode-api/reference/post-add-linode-disk)
    #[serde(rename = "ssh_keys", skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<Vec<String>>,
    /// __Read-only__ A boolean value indicating if the User has Two Factor Authentication (TFA) enabled. Run the [Create a two factor secret](https://techdocs.akamai.com/linode-api/reference/post-tfa-enable) operation to enable TFA.
    #[serde(rename = "tfa_enabled", skip_serializing_if = "Option::is_none")]
    pub tfa_enabled: Option<bool>,
    /// __Filterable__ The User's username. This is used for logging in, and may also be displayed alongside actions the User performs (for example, in Events or public StackScripts).
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// __Read-only__ The phone number verified for this User Profile with the [Verify a phone number](https://techdocs.akamai.com/linode-api/reference/post-profile-phone-number-verify) operation.  `null` if this User Profile has no verified phone number.
    #[serde(rename = "verified_phone_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub verified_phone_number: Option<Option<String>>,
}

impl User {
    /// A User on your Account. Unrestricted users can log in and access information about your Account, while restricted users may only access entities or perform actions they've been granted access to.
    pub fn new() -> User {
        User {
            email: None,
            last_login: None,
            password_created: None,
            restricted: None,
            ssh_keys: None,
            tfa_enabled: None,
            username: None,
            verified_phone_number: None,
        }
    }
}

