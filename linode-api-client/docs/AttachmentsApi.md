# \AttachmentsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_ticket_attachment**](AttachmentsApi.md#post_ticket_attachment) | **POST** /{apiVersion}/support/tickets/{ticketId}/attachments | Create a support ticket attachment



## post_ticket_attachment

> serde_json::Value post_ticket_attachment(api_version, ticket_id, file)
Create a support ticket attachment

Adds a file attachment to an open support ticket on your account. Use an attachment to help customer support resolve your ticket. The file attachment is submitted in the request as `multipart/form-data` type. Accepted file extensions include: `.gif`, `.jpg`, `.jpeg`, `.pjpg`, `.pjpeg`, `.tif`, `.tiff`, `.png`, `.pdf`, or `.txt`.   <<LB>>  ---   - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**ticket_id** | **i32** | The ID of the support ticket. | [required] |
**file** | **String** | The local, absolute path to the file you want to attach to your support ticket. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

