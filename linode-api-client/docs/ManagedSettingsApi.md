# \ManagedSettingsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_managed_linode_setting**](ManagedSettingsApi.md#get_managed_linode_setting) | **GET** /{apiVersion}/managed/linode-settings/{linodeId} | Get a Linode's managed settings
[**put_managed_linode_setting**](ManagedSettingsApi.md#put_managed_linode_setting) | **PUT** /{apiVersion}/managed/linode-settings/{linodeId} | Update a Linode's managed settings



## get_managed_linode_setting

> models::GetManagedLinodeSetting200Response get_managed_linode_setting(api_version, linode_id)
Get a Linode's managed settings

Returns a single Linode's Managed settings.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed linode-setting-view 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The Linode ID whose settings we are accessing. | [required] |

### Return type

[**models::GetManagedLinodeSetting200Response**](get_managed_linode_setting_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_managed_linode_setting

> models::PutManagedLinodeSetting200Response put_managed_linode_setting(api_version, linode_id, put_managed_linode_setting_request)
Update a Linode's managed settings

Updates a single Linode's Managed settings. This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed linode-setting-update \\   7833234 \\   --ssh.access true \\   --ssh.user linode \\   --ssh.port 22 \\   --ssh.ip 203.0.113.1     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The Linode ID whose settings we are accessing. | [required] |
**put_managed_linode_setting_request** | [**PutManagedLinodeSettingRequest**](PutManagedLinodeSettingRequest.md) | The settings to update. | [required] |

### Return type

[**models::PutManagedLinodeSetting200Response**](put_managed_linode_setting_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

