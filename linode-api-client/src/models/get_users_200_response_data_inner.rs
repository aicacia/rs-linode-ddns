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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUsers200ResponseDataInner {
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
    /// __Read-only__ If the user belongs to a [parent or child account](https://www.linode.com/docs/guides/parent-child-accounts/) relationship, this defines the user type in the respective account. Possible values include:  - `parent`. This is a user in an Akamai partner account. Akamai partners have a contractural relationship with their end customers, to sell Akamai services. This user can either have full access (a parent account admin user) or limited access. Limited users don't have access to manage child accounts, but they can be granted this access by an admin user.  - `child`. This is an Akamai partner's end customer user, in a child account. A child user can have either full or limited access. Full access lets the user manage other child users and the proxy user in a child account.  - `proxy`. This is a user on a child account that gives parent account users access to that child account. A parent account user with the `child_account_access` grant can [Create a proxy user token](https://techdocs.akamai.com/linode-api/reference/post-child-account-token) from the parent account. The parent user can use this token to run API operations from the child account, as if they were a child user.  - `default`. This applies to all regular, non-parent-child account users.
    #[serde(rename = "user_type", skip_serializing_if = "Option::is_none")]
    pub user_type: Option<UserTypeEnum>,
}

impl GetUsers200ResponseDataInner {
    pub fn new() -> GetUsers200ResponseDataInner {
        GetUsers200ResponseDataInner {
            email: None,
            last_login: None,
            password_created: None,
            restricted: None,
            ssh_keys: None,
            tfa_enabled: None,
            username: None,
            verified_phone_number: None,
            user_type: None,
        }
    }
}
/// __Read-only__ If the user belongs to a [parent or child account](https://www.linode.com/docs/guides/parent-child-accounts/) relationship, this defines the user type in the respective account. Possible values include:  - `parent`. This is a user in an Akamai partner account. Akamai partners have a contractural relationship with their end customers, to sell Akamai services. This user can either have full access (a parent account admin user) or limited access. Limited users don't have access to manage child accounts, but they can be granted this access by an admin user.  - `child`. This is an Akamai partner's end customer user, in a child account. A child user can have either full or limited access. Full access lets the user manage other child users and the proxy user in a child account.  - `proxy`. This is a user on a child account that gives parent account users access to that child account. A parent account user with the `child_account_access` grant can [Create a proxy user token](https://techdocs.akamai.com/linode-api/reference/post-child-account-token) from the parent account. The parent user can use this token to run API operations from the child account, as if they were a child user.  - `default`. This applies to all regular, non-parent-child account users.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserTypeEnum {
    #[serde(rename = "parent")]
    Parent,
    #[serde(rename = "child")]
    Child,
    #[serde(rename = "proxy")]
    Proxy,
    #[serde(rename = "default")]
    Default,
}

impl Default for UserTypeEnum {
    fn default() -> UserTypeEnum {
        Self::Parent
    }
}

