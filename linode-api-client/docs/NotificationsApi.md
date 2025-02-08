# \NotificationsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_notifications**](NotificationsApi.md#get_notifications) | **GET** /{apiVersion}/account/notifications | List notifications



## get_notifications

> models::GetNotifications200Response get_notifications(api_version)
List notifications

Returns a collection of notification objects that represent important, often time-sensitive details about your account. You can't interact directly with notifications, and a notification disappears when the circumstances that caused it have been resolved. For example, if you have an important ticket open, you can respond to that ticket to dismiss its notification.   <<LB>>  ---   - __CLI__.      ```     linode-cli account notifications-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetNotifications200Response**](get_notifications_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

