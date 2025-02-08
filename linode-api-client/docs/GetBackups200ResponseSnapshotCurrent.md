# GetBackups200ResponseSnapshotCurrent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available** | Option<**bool**> | __Read-only__ Whether this Backup is available for restoration.  Backups undergoing maintenance are not available for restoration. | [optional][readonly]
**configs** | Option<**Vec<String>**> | __Read-only__ A list of the labels of the Configuration profiles that are part of the Backup. | [optional][readonly]
**created** | Option<**String**> | __Read-only__ The date the Backup was taken. | [optional][readonly]
**disks** | Option<[**Vec<models::GetBackups200ResponseAutomaticInnerAllOfDisksInner>**](get_backups_200_response_automatic_inner_allOf_disks_inner.md)> | __Read-only__ A list of the disks that are part of the Backup. | [optional][readonly]
**finished** | Option<**String**> | __Read-only__ The date the Backup completed. | [optional][readonly]
**id** | Option<**i32**> | __Read-only__ The unique ID of this Backup. | [optional][readonly]
**label** | Option<**String**> | A label for Backups that are of type `snapshot`. | [optional]
**status** | Option<**String**> | __Read-only__ The current state of a specific Backup. | [optional][readonly]
**r#type** | Option<**String**> | __Read-only__ This indicates whether the Backup is an automatic Backup or manual snapshot taken by the User at a specific point in time. | [optional][readonly]
**updated** | Option<**String**> | __Read-only__ The date the Backup was most recently updated. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


