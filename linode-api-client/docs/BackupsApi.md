# \BackupsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_backup**](BackupsApi.md#get_backup) | **GET** /{apiVersion}/linode/instances/{linodeId}/backups/{backupId} | Get a backup
[**get_backups**](BackupsApi.md#get_backups) | **GET** /{apiVersion}/linode/instances/{linodeId}/backups | List backups
[**post_cancel_backups**](BackupsApi.md#post_cancel_backups) | **POST** /{apiVersion}/linode/instances/{linodeId}/backups/cancel | Cancel backups
[**post_enable_backups**](BackupsApi.md#post_enable_backups) | **POST** /{apiVersion}/linode/instances/{linodeId}/backups/enable | Enable backups
[**post_restore_backup**](BackupsApi.md#post_restore_backup) | **POST** /{apiVersion}/linode/instances/{linodeId}/backups/{backupId}/restore | Restore a backup
[**post_snapshot**](BackupsApi.md#post_snapshot) | **POST** /{apiVersion}/linode/instances/{linodeId}/backups | Create a snapshot



## get_backup

> models::GetBackups200ResponseSnapshotCurrent get_backup(api_version, linode_id, backup_id)
Get a backup

Returns information about a Backup.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes backup-view 123 123456     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The ID of the Linode the Backup belongs to. | [required] |
**backup_id** | **i32** | The ID of the Backup to look up. | [required] |

### Return type

[**models::GetBackups200ResponseSnapshotCurrent**](get_backups_200_response_snapshot_current.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_backups

> models::GetBackups200Response get_backups(api_version, linode_id)
List backups

Returns information about this Linode's available backups.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes backups-list 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The ID of the Linode the backups belong to. | [required] |

### Return type

[**models::GetBackups200Response**](get_backups_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_cancel_backups

> serde_json::Value post_cancel_backups(api_version, linode_id)
Cancel backups

Cancels the Backup service on the given Linode. Deletes all of this Linode's existing backups forever.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes backups-cancel 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The ID of the Linode to cancel backup service for. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_enable_backups

> serde_json::Value post_enable_backups(api_version, linode_id)
Enable backups

Enables backups for the specified Linode.  __Note__. Backups are not encrypted even when they are taken from an encrypted disk. When a backup is restored, and if encryption is enabled, the data stored on the disk is encrypted again.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes backups-enable 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The ID of the Linode to enable backup service for. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_restore_backup

> serde_json::Value post_restore_backup(api_version, linode_id, backup_id, post_restore_backup_request)
Restore a backup

Restores a Linode's Backup to the specified Linode.  The following conditions apply:    - Backups may not be restored across Regions.   - Only successfully completed Backups that are not undergoing maintenance can be restored.   - The Linode that the Backup is being restored to must not itself be in the process of creating a Backup.  __Note__. When you restore a backup, the restored disk is assigned the same [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier) as the original disk. In most cases, this is acceptable and does not cause issues. However, if you attempt to mount both the original disk and the corresponding restore disk at the same time (by assigning them both to devices in your Configuration Profile's __Block Device Assignment__), you will encounter a UUID \"collision\".  When this happens, the system selects, and mounts, only one of the disks at random. This is due to both disks sharing the same UUID, and your instance _may fail to boot_ since it will not be clear which disk is root. If your system does boot, you will not see any immediate indication if you are booted into the restored disk or the original disk, and you will be unable to access both disks at the same time.  To avoid this, we recommend only restoring a backup to the same Compute Instance if you do not intend on mounting them at the same time or are comfortable modifying UUIDs. If you need access to files on both the original disk and the restored disk simultaneously (such as needing to copy files between them), we suggest either restoring the backup to a separate Compute Instance or [creating](https://techdocs.akamai.com/linode-api/reference/post-linode-instance) a new Compute Instance with the desired `backup_id`.  To learn more about block device assignments and viewing your disks' UUIDs, see our guide on [Configuration Profiles](https://www.linode.com/docs/products/compute/compute-instances/guides/configuration-profiles/#block-device-assignment).  __Note__. Backups are not encrypted even when they are taken from an encrypted disk. When a backup is restored, and if encryption is enabled, the data stored on the disk is encrypted again.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes backup-restore 123 123456 \\   --linode_id 234 \\   --overwrite true     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The ID of the Linode that the Backup belongs to. | [required] |
**backup_id** | **i32** | The ID of the Backup to restore. | [required] |
**post_restore_backup_request** | [**PostRestoreBackupRequest**](PostRestoreBackupRequest.md) | Parameters to provide when restoring the Backup. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_snapshot

> models::GetBackups200ResponseSnapshotCurrent post_snapshot(api_version, linode_id, post_snapshot_request)
Create a snapshot

Creates a snapshot backup of a Linode.  __Note__. Backups are not encrypted even when they are taken from an encrypted disk. When a backup is restored, and if encryption is enabled, the data stored on the disk is encrypted again.  __Important__. If you already have a snapshot of this Linode, this is a destructive action. The previous snapshot will be deleted.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes snapshot 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | The ID of the Linode the backups belong to. | [required] |
**post_snapshot_request** | [**PostSnapshotRequest**](PostSnapshotRequest.md) |  | [required] |

### Return type

[**models::GetBackups200ResponseSnapshotCurrent**](get_backups_200_response_snapshot_current.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

