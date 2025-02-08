# \RepliesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_ticket_replies**](RepliesApi.md#get_ticket_replies) | **GET** /{apiVersion}/support/tickets/{ticketId}/replies | List replies
[**post_ticket_reply**](RepliesApi.md#post_ticket_reply) | **POST** /{apiVersion}/support/tickets/{ticketId}/replies | Create a reply



## get_ticket_replies

> models::GetTicketReplies200Response get_ticket_replies(api_version, ticket_id, page, page_size)
List replies

Returns a collection of replies to a support ticket on your account.   <<LB>>  ---   - __CLI__.      ```     linode-cli tickets replies 11223344     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**ticket_id** | **i32** | The ID of the support ticket. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetTicketReplies200Response**](get_ticket_replies_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_ticket_reply

> models::GetTicketReplies200ResponseDataInner post_ticket_reply(api_version, ticket_id, post_ticket_reply_request)
Create a reply

Adds a reply to an existing support ticket.   <<LB>>  ---   - __CLI__.      ```     linode-cli tickets reply 11223344 \\   --description \"Thank you for your help. I was able to figure out what the problem was and I successfully reset my password. You guys are the best!\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**ticket_id** | **i32** | The ID of the support ticket. | [required] |
**post_ticket_reply_request** | [**PostTicketReplyRequest**](PostTicketReplyRequest.md) |  | [required] |

### Return type

[**models::GetTicketReplies200ResponseDataInner**](get_ticket_replies_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

