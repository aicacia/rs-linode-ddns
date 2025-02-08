# \TagsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_ag**](TagsApi.md#delete_ag) | **DELETE** /{apiVersion}/tags/{tagLabel} | Delete a tag
[**get_tagged_objects**](TagsApi.md#get_tagged_objects) | **GET** /{apiVersion}/tags/{tagLabel} | List tagged objects
[**get_tags**](TagsApi.md#get_tags) | **GET** /{apiVersion}/tags | List tags
[**post_tag**](TagsApi.md#post_tag) | **POST** /{apiVersion}/tags | Create a tag



## delete_ag

> serde_json::Value delete_ag(api_version, tag_label)
Delete a tag

Remove a Tag from all objects and delete it.  __Important__. You must be an unrestricted User in order to access, add, or modify Tags information.   <<LB>>  ---   - __CLI__.      ```     linode-cli tags delete linode-cli tags rm     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**tag_label** | **String** | The `label` of the tag to access. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tagged_objects

> models::GetTaggedObjects200Response get_tagged_objects(api_version, tag_label, page, page_size)
List tagged objects

Returns a paginated list of all objects you've tagged with the requested Tag. This is a mixed collection of all object types.  __Important__. You must be an unrestricted User in order to access, add, or modify Tags information.   <<LB>>  ---   - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**tag_label** | **String** | The `label` of the tag to access. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetTaggedObjects200Response**](get_tagged_objects_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tags

> models::GetTags200Response get_tags(api_version, page, page_size)
List tags

Tags are User-defined labels attached to objects in your Account, such as Linodes. They are used for specifying and grouping attributes of objects that are relevant to the User.  This operation returns a paginated list of Tags on your account.  __Important__. You must be an unrestricted User in order to access, add, or modify Tags information.   <<LB>>  ---   - __CLI__.      ```     linode-cli tags list linode-cli tags ls     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetTags200Response**](get_tags_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_tag

> models::GetTags200ResponseDataInner post_tag(api_version, post_tag_request)
Create a tag

Creates a new Tag and optionally tags requested objects with it immediately.  __Important__. You must be an unrestricted User in order to access, add, or modify Tags information.   <<LB>>  ---   - __CLI__.      ```     linode-cli tags create \\   --label 'example tag' \\   --linodes 123 \\   --linodes 456 \\   --volumes 9082 \\   --volumes 10003     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_tag_request** | Option<[**PostTagRequest**](PostTagRequest.md)> | The tag to create, and optionally the objects to tag. |  |

### Return type

[**models::GetTags200ResponseDataInner**](get_tags_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

