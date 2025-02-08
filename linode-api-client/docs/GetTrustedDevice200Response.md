# GetTrustedDevice200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | Option<**String**> | __Read-only__ When this Remember Me session was started.  This corresponds to the time of login with the \"Remember Me\" box checked. | [optional][readonly]
**expiry** | Option<**String**> | __Read-only__ When this TrustedDevice session expires.  Sessions typically last 30 days. | [optional][readonly]
**id** | Option<**i32**> | __Read-only__ The unique ID for this TrustedDevice. | [optional][readonly]
**last_authenticated** | Option<**String**> | __Read-only__ The last time this TrustedDevice was successfully used to authenticate to [login.linode.com](https://login.linode.com). | [optional][readonly]
**last_remote_addr** | Option<**String**> | __Read-only__ The last IP Address to successfully authenticate with this TrustedDevice. | [optional][readonly]
**user_agent** | Option<**String**> | __Read-only__ The User Agent of the browser that created this TrustedDevice session. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


