# \PlansApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_longview_plan**](PlansApi.md#get_longview_plan) | **GET** /{apiVersion}/longview/plan | Get a Longview plan
[**put_longview_plan**](PlansApi.md#put_longview_plan) | **PUT** /{apiVersion}/longview/plan | Update a Longview plan



## get_longview_plan

> models::GetLongviewPlan200Response get_longview_plan(api_version)
Get a Longview plan

Get the details of your current Longview plan. This returns a `LongviewSubscription` object for your current Longview Pro plan, or an empty set `{}` if your current plan is Longview Free.  You must have at least one of the following `global` [List a user's grants](https://techdocs.akamai.com/linode-api/reference/get-user-grants) in order to run this operation:    - `\"account_access\": read_write`   - `\"account_access\": read_only`   - `\"longview_subscription\": true`   - `\"add_longview\": true`  To update your subscription plan, send a request to [Update a Longview plan](https://techdocs.akamai.com/linode-api/reference/put-longview-plan).   <<LB>>  ---   - __CLI__.      ```     linode-cli longview plan-view     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     longview:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetLongviewPlan200Response**](get_longview_plan_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_longview_plan

> models::GetLongviewPlan200Response put_longview_plan(api_version, put_longview_plan_request)
Update a Longview plan

Update your Longview plan to that of the given subscription ID. This returns a `LongviewSubscription` object for the updated Longview Pro plan, or an empty set `{}` if the updated plan is Longview Free.  You must have `\"longview_subscription\": true` configured as a `global` [user grant](https://techdocs.akamai.com/linode-api/reference/get-user-grants) in order to run this operation.  You can send a request to the [List Longview subscriptions](https://techdocs.akamai.com/linode-api/reference/get-longview-subscriptions) operation to receive the details, including `id`'s, of each plan.   <<LB>>  ---   - __CLI__.      ```     linode-cli longview plan-update --longview_subscription longview-10     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     longview:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**put_longview_plan_request** | [**PutLongviewPlanRequest**](PutLongviewPlanRequest.md) | Update your Longview subscription plan. | [required] |

### Return type

[**models::GetLongviewPlan200Response**](get_longview_plan_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

