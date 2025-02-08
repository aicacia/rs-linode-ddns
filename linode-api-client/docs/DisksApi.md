# \DisksApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_disk**](DisksApi.md#delete_disk) | **DELETE** /{apiVersion}/linode/instances/{linodeId}/disks/{diskId} | Delete a disk
[**get_linode_disk**](DisksApi.md#get_linode_disk) | **GET** /{apiVersion}/linode/instances/{linodeId}/disks/{diskId} | Get a disk
[**get_linode_disks**](DisksApi.md#get_linode_disks) | **GET** /{apiVersion}/linode/instances/{linodeId}/disks | List disks
[**post_add_linode_disk**](DisksApi.md#post_add_linode_disk) | **POST** /{apiVersion}/linode/instances/{linodeId}/disks | Create a disk
[**post_clone_linode_disk**](DisksApi.md#post_clone_linode_disk) | **POST** /{apiVersion}/linode/instances/{linodeId}/disks/{diskId}/clone | Clone a disk
[**post_reset_disk_password**](DisksApi.md#post_reset_disk_password) | **POST** /{apiVersion}/linode/instances/{linodeId}/disks/{diskId}/password | Reset a disk root password
[**post_resize_disk**](DisksApi.md#post_resize_disk) | **POST** /{apiVersion}/linode/instances/{linodeId}/disks/{diskId}/resize | Resize a disk
[**put_disk**](DisksApi.md#put_disk) | **PUT** /{apiVersion}/linode/instances/{linodeId}/disks/{diskId} | Update a disk



## delete_disk

> serde_json::Value delete_disk(api_version, linode_id, disk_id)
Delete a disk

Deletes a Disk you have permission to `read_write`.  __Deleting a Disk is a destructive action and cannot be undone.__   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes disk-delete 123 24674     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**disk_id** | **i32** | ID of the Disk to look up. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_disk

> models::GetLinodeDisks200ResponseDataInner get_linode_disk(api_version, linode_id, disk_id)
Get a disk

View Disk information for a Disk associated with this Linode.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes disk-view 123 25674     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**disk_id** | **i32** | ID of the Disk to look up. | [required] |

### Return type

[**models::GetLinodeDisks200ResponseDataInner**](get_linode_disks_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_disks

> models::GetLinodeDisks200Response get_linode_disks(api_version, linode_id, page, page_size)
List disks

View Disk information for Disks associated with this Linode.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes disks-list 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetLinodeDisks200Response**](get_linode_disks_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_add_linode_disk

> models::GetLinodeDisks200ResponseDataInner post_add_linode_disk(api_version, linode_id, post_add_linode_disk_request)
Create a disk

Add a new disk to an existing Linode. You can create an empty disk to manually configure it later. You can also target a stored `image` to build the disk using a pre-configured file system.  - A Linode can have up to 50 disks.  - When creating an empty disk, you need to provide a `label` for it. If you don't include a `label`, you need to target an `image` instead.  - When you create a disk from an `image`, you need to set a `root_pass` for the disk.  - The default file system for a new disk is `ext4`. If you're creating one from an `image`, the disk inherits the file system of that `image`, is unless you specify otherwise.  - When you deploy a StackScript on a disk:    - You can run [List StackScripts](https://techdocs.akamai.com/linode-api/reference/get-stack-scripts) to review available StackScripts.    - You need to include a compatible `image` when creating the disk. Run [Get a StackScript](https://techdocs.akamai.com/linode-api/reference/get-stack-script) to review compatible images.    - You should supply SSH keys for the disk's root user, using the `authorized_keys` field.    - You can include individual users via the `authorized_users` field. Before you can add a user, it needs an SSH key assigned to its profile. See [Add an SSH key](https://techdocs.akamai.com/linode-api/reference/post-add-ssh-key) for more information.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes disk-create 123 \\   --size 1300 \\   --authorized_keys \"ssh-rsa AAAA_valid_public_ssh_key_123456785== user@their-computer\" \\   --authorized_users \"myUser\" \\   --authorized_users \"secondaryUser\" \\   --root_pass aComplex@Password \\   --image \"linode/debian9\" \\   --stackscript_id 10079 \\   --stackscript_data '{\"gh_username\": \"linode\"}'     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**post_add_linode_disk_request** | [**PostAddLinodeDiskRequest**](PostAddLinodeDiskRequest.md) | The parameters to set when creating the Disk. | [required] |

### Return type

[**models::GetLinodeDisks200ResponseDataInner**](get_linode_disks_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_clone_linode_disk

> models::GetLinodeDisks200ResponseDataInner post_clone_linode_disk(api_version, linode_id, disk_id)
Clone a disk

Copies a disk, byte-for-byte, into a new disk on the same Linode. The operation fails if the target doesn't have enough storage space. A Linode can have up to 50 disks.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes disk-clone     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**disk_id** | **i32** | ID of the Disk to clone. | [required] |

### Return type

[**models::GetLinodeDisks200ResponseDataInner**](get_linode_disks_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_reset_disk_password

> serde_json::Value post_reset_disk_password(api_version, linode_id, disk_id, post_reset_disk_password_request)
Reset a disk root password

Resets the password of a Disk you have permission to `read_write`.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes disk-reset-password \\   123 25674 \\   --password aComplex@Password     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**disk_id** | **i32** | ID of the Disk to look up. | [required] |
**post_reset_disk_password_request** | [**PostResetDiskPasswordRequest**](PostResetDiskPasswordRequest.md) | The new password. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_resize_disk

> serde_json::Value post_resize_disk(api_version, linode_id, disk_id, post_resize_disk_request)
Resize a disk

Resizes a Disk you have permission to `read_write`.  The Disk must not be in use. If the Disk is in use, the request will succeed but the resize will ultimately fail. For a request to succeed, the Linode must be shut down prior to resizing the Disk, or the Disk must not be assigned to the Linode's active Configuration Profile.  If you are resizing the Disk to a smaller size, it cannot be made smaller than what is required by the total size of the files current on the Disk.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes disk-resize 123 25674 \\   --size 2048     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**disk_id** | **i32** | ID of the Disk to look up. | [required] |
**post_resize_disk_request** | [**PostResizeDiskRequest**](PostResizeDiskRequest.md) | The new size of the Disk. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_disk

> models::GetLinodeDisks200ResponseDataInner put_disk(api_version, linode_id, disk_id, put_disk_request)
Update a disk

Updates a Disk that you have permission to `read_write`.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes disk-update 123 25674 \\   --label \"Debian 9 Disk\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**disk_id** | **i32** | ID of the Disk to look up. | [required] |
**put_disk_request** | [**PutDiskRequest**](PutDiskRequest.md) | Updates the parameters of a single Disk. | [required] |

### Return type

[**models::GetLinodeDisks200ResponseDataInner**](get_linode_disks_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

