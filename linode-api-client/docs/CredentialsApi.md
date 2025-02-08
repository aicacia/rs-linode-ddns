# \CredentialsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_databases_mysql_instance_credentials**](CredentialsApi.md#get_databases_mysql_instance_credentials) | **GET** /{apiVersion}/databases/mysql/instances/{instanceId}/credentials | Get MySQL Managed Database credentials
[**get_databases_postgre_sql_instance_credentials**](CredentialsApi.md#get_databases_postgre_sql_instance_credentials) | **GET** /{apiVersion}/databases/postgresql/instances/{instanceId}/credentials | Get PostgreSQL Managed Database credentials
[**get_managed_credential**](CredentialsApi.md#get_managed_credential) | **GET** /{apiVersion}/managed/credentials/{credentialId} | Get a managed credential
[**get_managed_credentials**](CredentialsApi.md#get_managed_credentials) | **GET** /{apiVersion}/managed/credentials | List managed credentials
[**post_databases_mysql_instance_credentials_reset**](CredentialsApi.md#post_databases_mysql_instance_credentials_reset) | **POST** /{apiVersion}/databases/mysql/instances/{instanceId}/credentials/reset | Reset MySQL Managed Database credentials
[**post_databases_postgre_sql_instance_credentials_reset**](CredentialsApi.md#post_databases_postgre_sql_instance_credentials_reset) | **POST** /{apiVersion}/databases/postgresql/instances/{instanceId}/credentials/reset | Reset PostgreSQL Managed Database credentials
[**post_managed_credential**](CredentialsApi.md#post_managed_credential) | **POST** /{apiVersion}/managed/credentials | Create a managed credential
[**post_managed_credential_revoke**](CredentialsApi.md#post_managed_credential_revoke) | **POST** /{apiVersion}/managed/credentials/{credentialId}/revoke | Delete a managed credential
[**post_managed_credential_username_password**](CredentialsApi.md#post_managed_credential_username_password) | **POST** /{apiVersion}/managed/credentials/{credentialId}/update | Update a managed credential's username and password
[**put_managed_credential**](CredentialsApi.md#put_managed_credential) | **PUT** /{apiVersion}/managed/credentials/{credentialId} | Update a managed credential



## get_databases_mysql_instance_credentials

> models::GetDatabasesMysqlInstanceCredentials200Response get_databases_mysql_instance_credentials(api_version, instance_id)
Get MySQL Managed Database credentials

Display the root username and password for an accessible MySQL Managed Database. The database's status needs to be `active`.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases mysql-creds-view 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**instance_id** | **i32** | The ID of the Managed PostgreSQL Database. | [required] |

### Return type

[**models::GetDatabasesMysqlInstanceCredentials200Response**](get_databases_mysql_instance_credentials_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_databases_postgre_sql_instance_credentials

> models::GetDatabasesMysqlInstanceCredentials200Response get_databases_postgre_sql_instance_credentials(api_version, instance_id)
Get PostgreSQL Managed Database credentials

Display the root username and password for an accessible PostgreSQL Managed Database. The database's status needs to be `active`.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases postgresql-creds-view 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**instance_id** | **i32** | The ID of the Managed PostgreSQL Database. | [required] |

### Return type

[**models::GetDatabasesMysqlInstanceCredentials200Response**](get_databases_mysql_instance_credentials_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_managed_credential

> models::GetManagedCredentials200ResponseDataInner get_managed_credential(api_version, credential_id)
Get a managed credential

Returns a single Managed Credential.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed credential-view 9991     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**credential_id** | **i32** | The ID of the Credential to access. | [required] |

### Return type

[**models::GetManagedCredentials200ResponseDataInner**](get_managed_credentials_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_managed_credentials

> models::GetManagedCredentials200Response get_managed_credentials(api_version, page, page_size)
List managed credentials

Returns a paginated list of Managed Credentials on your Account.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed credentials-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetManagedCredentials200Response**](get_managed_credentials_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_databases_mysql_instance_credentials_reset

> serde_json::Value post_databases_mysql_instance_credentials_reset(api_version, instance_id)
Reset MySQL Managed Database credentials

Reset the root password for a MySQL Managed Database. A new root password is randomly generated and accessible with the [Get MySQL Managed Database credentials](https://techdocs.akamai.com/linode-api/reference/get-databases-mysql-instance-credentials) operation.  - The database's status needs to be `active`.  - Only unrestricted users can access this operation. These users have access regardless of the acting token's OAuth scopes.  - It may take several seconds for credentials to reset.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases mysql-creds-reset 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**instance_id** | **i32** | The ID of the Managed MySQL Database. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_databases_postgre_sql_instance_credentials_reset

> serde_json::Value post_databases_postgre_sql_instance_credentials_reset(api_version, instance_id)
Reset PostgreSQL Managed Database credentials

Reset the root password for a PostgreSQL Managed Database. A new root password is randomly generated and accessible with the [Get PostgreSQL Managed Database credentials](https://techdocs.akamai.com/linode-api/reference/get-databases-postrge-sql-instance-credentials) operation.  - The database's status needs to be `active`.  - Only unrestricted users can access this operation. These users have access regardless of the acting token's OAuth scopes.  - It may take several seconds for credentials to reset.   <<LB>>  ---   - __CLI__.      ```     linode-cli databases postgresql-creds-reset 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     databases:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**instance_id** | **i32** | The ID of the Managed PostgreSQL Database. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_managed_credential

> models::GetManagedCredentials200ResponseDataInner post_managed_credential(api_version, post_managed_credential_request)
Create a managed credential

Creates a Managed Credential. A Managed Credential is stored securely to allow Linode special forces to access your Managed Services and resolve issues.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed credential-create \\   --label prod-password-1 \\   --username johndoe \\   --password s3cur3P@ssw0rd     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_managed_credential_request** | Option<[**PostManagedCredentialRequest**](PostManagedCredentialRequest.md)> | Information about the Credential to create. |  |

### Return type

[**models::GetManagedCredentials200ResponseDataInner**](get_managed_credentials_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_managed_credential_revoke

> serde_json::Value post_managed_credential_revoke(api_version, credential_id)
Delete a managed credential

Deletes a Managed Credential.  Linode special forces will no longer have access to this Credential when attempting to resolve issues.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed credential-revoke 9991     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**credential_id** | **i32** | The ID of the Credential to access. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_managed_credential_username_password

> serde_json::Value post_managed_credential_username_password(api_version, credential_id, post_managed_credential_username_password_request)
Update a managed credential's username and password

Updates the username and password for a Managed Credential.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed credential-update-username-password 9991 \\   --username johndoe \\   --password s3cur3P@ssw0rd     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**credential_id** | **i32** | The ID of the Credential to update. | [required] |
**post_managed_credential_username_password_request** | Option<[**PostManagedCredentialUsernamePasswordRequest**](PostManagedCredentialUsernamePasswordRequest.md)> | The new username and password to assign to the Managed Credential. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_managed_credential

> models::GetManagedCredentials200ResponseDataInner put_managed_credential(api_version, credential_id, put_managed_credential_request)
Update a managed credential

Updates the label of a Managed Credential. This operation does not update the username and password for a Managed Credential. To do this, run the [Update a managed credential's username and password](https://techdocs.akamai.com/linode-api/reference/post-managed-credential-username-password)) operation instead. This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed credential-update 9991 \\   --label prod-password-1     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**credential_id** | **i32** | The ID of the Credential to access. | [required] |
**put_managed_credential_request** | [**PutManagedCredentialRequest**](PutManagedCredentialRequest.md) | The fields to update. | [required] |

### Return type

[**models::GetManagedCredentials200ResponseDataInner**](get_managed_credentials_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

