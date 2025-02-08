# \PromoCreditsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_promo_credit**](PromoCreditsApi.md#post_promo_credit) | **POST** /{apiVersion}/account/promo-codes | Add a promo credit



## post_promo_credit

> models::GetAccount200ResponseActivePromotionsInner post_promo_credit(api_version, post_promo_credit_request)
Add a promo credit

Adds an expiring Promo Credit to your account. The following restrictions apply:  - Your account needs to be less than 90 days old.  - You can't already have a Promo Credit on your account.  - The user making the request needs to be unrestricted. You can run the [Update a user](https://techdocs.akamai.com/linode-api/reference/put-user) operation to change a user's restricted status.  - The `promo_code` needs to be valid and unexpired.  __Parent and child accounts__  In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:  - Child account users can't run this operation. These users don't have access to billing-related operations.   <<LB>>  ---   - __CLI__.      ```     linode-cli account \\   promo-add \\   --promo-code abcdefABCDEF1234567890     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_promo_credit_request** | Option<[**PostPromoCreditRequest**](PostPromoCreditRequest.md)> | Enter a Promo Code to add its associated credit to your Account. |  |

### Return type

[**models::GetAccount200ResponseActivePromotionsInner**](get_account_200_response_active_promotions_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

