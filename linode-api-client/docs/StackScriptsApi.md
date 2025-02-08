# \StackScriptsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_stack_script**](StackScriptsApi.md#delete_stack_script) | **DELETE** /{apiVersion}/linode/stackscripts/{stackscriptId} | Delete a StackScript
[**get_stack_script**](StackScriptsApi.md#get_stack_script) | **GET** /{apiVersion}/linode/stackscripts/{stackscriptId} | Get a StackScript
[**get_stack_scripts**](StackScriptsApi.md#get_stack_scripts) | **GET** /{apiVersion}/linode/stackscripts | List StackScripts
[**post_add_stack_script**](StackScriptsApi.md#post_add_stack_script) | **POST** /{apiVersion}/linode/stackscripts | Create a StackScript
[**put_stack_script**](StackScriptsApi.md#put_stack_script) | **PUT** /{apiVersion}/linode/stackscripts/{stackscriptId} | Update a StackScript



## delete_stack_script

> serde_json::Value delete_stack_script(api_version, stackscript_id)
Delete a StackScript

Deletes a private StackScript you have permission to `read_write`. You cannot delete a public StackScript.   <<LB>>  ---   - __CLI__.      ```     linode-cli stackscripts delete 10079     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     stackscripts:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**stackscript_id** | **String** | The ID of the StackScript to look up. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stack_script

> models::GetStackScripts200ResponseDataInner get_stack_script(api_version, stackscript_id)
Get a StackScript

Returns all of the information about a specified StackScript, including the contents of the script.   <<LB>>  ---   - __CLI__.      ```     linode-cli stackscripts view 10079     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     stackscripts:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**stackscript_id** | **String** | The ID of the StackScript to look up. | [required] |

### Return type

[**models::GetStackScripts200ResponseDataInner**](get_stack_scripts_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stack_scripts

> models::GetStackScripts200Response get_stack_scripts(api_version, page, page_size)
List StackScripts

If the request is not authenticated, only public StackScripts are returned.  For more information on StackScripts, please read our [StackScripts documentation](https://www.linode.com/docs/products/tools/stackscripts/).   <<LB>>  ---   - __CLI__.      ```     linode-cli stackscripts list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     stackscripts:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetStackScripts200Response**](get_stack_scripts_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_add_stack_script

> models::GetStackScripts200ResponseDataInner post_add_stack_script(api_version, post_add_stack_script_request)
Create a StackScript

Creates a StackScript in your Account.   <<LB>>  ---   - __CLI__.      ```     linode-cli stackscripts create \\   --label a-stackscript \\   --description \"This StackScript install and configures MySQL\" \\   --images \"linode/debian9\" \\   --images \"linode/debian8\" \\   --is_public true \\   --rev_note \"Set up MySQL\" \\   --script '#!/bin/bash'     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     stackscripts:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_add_stack_script_request** | [**PostAddStackScriptRequest**](PostAddStackScriptRequest.md) | The properties to set for the new StackScript. | [required] |

### Return type

[**models::GetStackScripts200ResponseDataInner**](get_stack_scripts_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_stack_script

> models::GetStackScripts200ResponseDataInner put_stack_script(api_version, stackscript_id, put_stack_script_request)
Update a StackScript

Updates a StackScript.  __Once a StackScript is made public, it cannot be made private.__   <<LB>>  ---   - __CLI__.      ```     linode-cli stackscripts update 10079 \\   --label a-stackscript \\   --description \"This StackScript installs \\     and configures MySQL\" \\   --images \"linode/debian9\" \\   --images \"linode/debian8\" \\   --is_public true \\   --rev_note \"Set up MySQL\" \\   --script '#!/bin/bash'     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     stackscripts:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**stackscript_id** | **String** | The ID of the StackScript to look up. | [required] |
**put_stack_script_request** | Option<[**PutStackScriptRequest**](PutStackScriptRequest.md)> | The fields to update. |  |

### Return type

[**models::GetStackScripts200ResponseDataInner**](get_stack_scripts_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

