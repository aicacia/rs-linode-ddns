# \SubscriptionsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_longview_subscription**](SubscriptionsApi.md#get_longview_subscription) | **GET** /{apiVersion}/longview/subscriptions/{subscriptionId} | Get a Longview subscription
[**get_longview_subscriptions**](SubscriptionsApi.md#get_longview_subscriptions) | **GET** /{apiVersion}/longview/subscriptions | List Longview subscriptions



## get_longview_subscription

> models::GetLongviewPlan200Response get_longview_subscription(api_version, subscription_id)
Get a Longview subscription

Get the Longview plan details as a single `LongviewSubscription` object for the provided subscription ID. This is a public endpoint and requires no authentication.   <<LB>>  ---   - __CLI__.      ```     linode-cli longview subscription-view     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**subscription_id** | **String** | The Longview Subscription to look up. | [required] |

### Return type

[**models::GetLongviewPlan200Response**](get_longview_plan_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_longview_subscriptions

> models::GetLongviewSubscriptions200Response get_longview_subscriptions(api_version, page, page_size)
List Longview subscriptions

Returns a paginated list of available Longview Subscriptions. This is a public endpoint and requires no authentication.   <<LB>>  ---   - __CLI__.      ```     linode-cli longview subscriptions-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetLongviewSubscriptions200Response**](get_longview_subscriptions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

