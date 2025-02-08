# \SshKeysApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_ssh_key**](SshKeysApi.md#delete_ssh_key) | **DELETE** /{apiVersion}/profile/sshkeys/{sshKeyId} | Delete an SSH key
[**get_ssh_key**](SshKeysApi.md#get_ssh_key) | **GET** /{apiVersion}/profile/sshkeys/{sshKeyId} | Get an SSH key
[**get_ssh_keys**](SshKeysApi.md#get_ssh_keys) | **GET** /{apiVersion}/profile/sshkeys | List SSH keys
[**post_add_ssh_key**](SshKeysApi.md#post_add_ssh_key) | **POST** /{apiVersion}/profile/sshkeys | Add an SSH key
[**put_ssh_key**](SshKeysApi.md#put_ssh_key) | **PUT** /{apiVersion}/profile/sshkeys/{sshKeyId} | Update an SSH key



## delete_ssh_key

> serde_json::Value delete_ssh_key(api_version, ssh_key_id)
Delete an SSH key

Deletes an SSH Key you have access to.  __Note__. deleting an SSH Key will _not_ remove it from any Linode or Disk that was deployed with `authorized_keys`. In those cases, the keys must be manually deleted on the Linode or Disk. This operation will only delete the key's association from your Profile.   <<LB>>  ---   - __CLI__.      ```     linode-cli sshkeys delete 42     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**ssh_key_id** | **i32** | The ID of the SSHKey. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ssh_key

> models::GetSshKeys200ResponseDataInner get_ssh_key(api_version, ssh_key_id)
Get an SSH key

Returns a single SSH Key object identified by `id` that you have access to view.   <<LB>>  ---   - __CLI__.      ```     linode-cli sshkeys view 42     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**ssh_key_id** | **i32** | The ID of the SSHKey. | [required] |

### Return type

[**models::GetSshKeys200ResponseDataInner**](get_ssh_keys_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ssh_keys

> models::GetSshKeys200Response get_ssh_keys(api_version, page, page_size)
List SSH keys

Returns a collection of SSH Keys you've added to your Profile.   <<LB>>  ---   - __CLI__.      ```     linode-cli sshkeys list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetSshKeys200Response**](get_ssh_keys_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_add_ssh_key

> models::GetSshKeys200ResponseDataInner post_add_ssh_key(api_version, post_add_ssh_key_request)
Add an SSH key

Adds an SSH Key to your Account profile.   <<LB>>  ---   - __CLI__.      ```     linode-cli sshkeys create \\   --label \"My SSH Key\" \\   --ssh_key \"ssh-rsa AAAA_valid_public_ssh_key_123456785== user@their-computer\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_add_ssh_key_request** | Option<[**PostAddSshKeyRequest**](PostAddSshKeyRequest.md)> | Add SSH Key. |  |

### Return type

[**models::GetSshKeys200ResponseDataInner**](get_ssh_keys_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_ssh_key

> models::GetSshKeys200ResponseDataInner put_ssh_key(api_version, ssh_key_id, put_ssh_key_request)
Update an SSH key

Updates an SSH Key that you have permission to `read_write`.  Only SSH key labels can be updated.   <<LB>>  ---   - __CLI__.      ```     linode-cli sshkeys update 42 \\   --label \"my laptop\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**ssh_key_id** | **i32** | The ID of the SSHKey. | [required] |
**put_ssh_key_request** | [**PutSshKeyRequest**](PutSshKeyRequest.md) | The fields to update. | [required] |

### Return type

[**models::GetSshKeys200ResponseDataInner**](get_ssh_keys_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

