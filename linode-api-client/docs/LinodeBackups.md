# LinodeBackups

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available** | Option<**bool**> | __Read-only__ Whether Backups for this Linode are available for restoration.  Backups undergoing maintenance are not available for restoration. | [optional][readonly]
**enabled** | Option<**bool**> | __Read-only__ If this Linode has the Backup service enabled. To enable backups, run [Enable backups](https://techdocs.akamai.com/linode-api/reference/post-enable-backups). | [optional][readonly]
**last_successful** | Option<**String**> | __Read-only__ The last successful backup time. Displayed as `null` if there was no previous backup. | [optional][readonly]
**schedule** | Option<[**models::LinodeBackupsSchedule**](Linode_backups_schedule.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


