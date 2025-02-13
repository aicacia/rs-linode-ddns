# GetAccountSettings200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backups_enabled** | Option<**bool**> | Account-wide backups default.  If `true`, all Linodes created will automatically be enrolled in the Backups service.  If `false`, Linodes will not be enrolled by default, but may still be enrolled on creation or later. | [optional]
**longview_subscription** | Option<**String**> | __Read-only__ The Longview Pro tier you are currently subscribed to. The value must be a [Longview subscription](https://techdocs.akamai.com/linode-api/reference/get-longview-subscriptions) ID or `null` for Longview Free. | [optional][readonly]
**managed** | Option<**bool**> | __Read-only__ Our 24/7 incident response service. This robust, multi-homed monitoring system distributes monitoring checks to ensure that your servers remain online and available at all times. Linode Managed can monitor any service or software stack reachable over TCP or HTTP. Once you add a service to Linode Managed, we'll monitor it for connectivity, response, and total request time. | [optional][readonly]
**network_helper** | Option<**bool**> | Enables network helper across all users by default for new Linodes and Linode Configs. | [optional]
**object_storage** | Option<**String**> | __Read-only__ A string describing the status of this account's Object Storage service enrollment. | [optional][readonly][default to Disabled]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


