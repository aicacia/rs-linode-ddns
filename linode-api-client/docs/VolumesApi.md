# \VolumesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_volume**](VolumesApi.md#delete_volume) | **DELETE** /{apiVersion}/volumes/{volumeId} | Delete a volume
[**get_linode_volumes**](VolumesApi.md#get_linode_volumes) | **GET** /{apiVersion}/linode/instances/{linodeId}/volumes | List a Linode's volumes
[**get_volume**](VolumesApi.md#get_volume) | **GET** /{apiVersion}/volumes/{volumeId} | Get a volume
[**get_volumes**](VolumesApi.md#get_volumes) | **GET** /{apiVersion}/volumes | List volumes
[**post_attach_volume**](VolumesApi.md#post_attach_volume) | **POST** /{apiVersion}/volumes/{volumeId}/attach | Attach a volume
[**post_clone_volume**](VolumesApi.md#post_clone_volume) | **POST** /{apiVersion}/volumes/{volumeId}/clone | Clone a volume
[**post_detach_volume**](VolumesApi.md#post_detach_volume) | **POST** /{apiVersion}/volumes/{volumeId}/detach | Detach a volume
[**post_resize_volume**](VolumesApi.md#post_resize_volume) | **POST** /{apiVersion}/volumes/{volumeId}/resize | Resize a volume
[**post_volume**](VolumesApi.md#post_volume) | **POST** /{apiVersion}/volumes | Create a volume
[**put_volume**](VolumesApi.md#put_volume) | **PUT** /{apiVersion}/volumes/{volumeId} | Update a volume



## delete_volume

> serde_json::Value delete_volume(api_version, volume_id)
Delete a volume

Deletes a Volume you have permission to `read_write`.  - __Deleting a Volume is a destructive action and cannot be undone.__  - Deleting stops billing for the Volume. You will be billed for time used within the billing period the Volume was active.  - Volumes that are migrating cannot be deleted until the migration is finished.   <<LB>>  ---   - __CLI__.      ```     linode-cli volumes delete 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     volumes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**volume_id** | **i32** | ID of the Volume to look up. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_volumes

> models::GetLinodeVolumes200Response get_linode_volumes(api_version, linode_id, page, page_size)
List a Linode's volumes

View Block Storage Volumes attached to this Linode.   <<LB>>  ---   - __CLI__.      ```     linode-cli linodes volumes 123     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetLinodeVolumes200Response**](get_linode_volumes_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_volume

> models::Volume get_volume(api_version, volume_id, page, page_size)
Get a volume

Get information about a single Volume.   <<LB>>  ---   - __CLI__.      ```     linode-cli volumes view 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     volumes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**volume_id** | **i32** | ID of the Volume to look up. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::Volume**](Volume.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_volumes

> models::GetVolumes200Response get_volumes(api_version, page, page_size)
List volumes

Returns a paginated list of Volumes you have permission to view.   <<LB>>  ---   - __CLI__.      ```     linode-cli volumes list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     volumes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetVolumes200Response**](get_volumes_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_attach_volume

> models::Volume post_attach_volume(api_version, volume_id, post_attach_volume_request)
Attach a volume

Attaches a Volume on your Account to an existing Linode on your Account. In order for this request to complete successfully, your User must have `read_write` permission to the Volume and `read_write` permission to the Linode. Additionally, the Volume and Linode must be located in the same Region.   <<LB>>  ---   - __CLI__.      ```     linode-cli volumes attach 12345 \\   --linode_id 12346 \\   --config_id 23456     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     volumes:read_write linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**volume_id** | **i32** | ID of the Volume to attach. | [required] |
**post_attach_volume_request** | [**PostAttachVolumeRequest**](PostAttachVolumeRequest.md) | Volume to attach to a Linode. | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_clone_volume

> models::Volume post_clone_volume(api_version, volume_id, post_clone_volume_request)
Clone a volume

Creates a Volume on your Account. In order for this request to complete successfully, your User must have the `add_volumes` grant. The new Volume will have the same size and data as the source Volume. Creating a new Volume will incur a charge on your Account.  - Only Volumes with a `status` of `active` can be cloned.   <<LB>>  ---   - __CLI__.      ```     linode-cli volumes clone 12345 \\   --label my-volume     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     volumes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**volume_id** | **i32** | ID of the Volume to clone. | [required] |
**post_clone_volume_request** | [**PostCloneVolumeRequest**](PostCloneVolumeRequest.md) | The requested state your Volume will be cloned into. | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_detach_volume

> serde_json::Value post_detach_volume(api_version, volume_id)
Detach a volume

Detaches a Volume on your Account from a Linode on your Account. In order for this request to complete successfully, your User must have `read_write` access to the Volume and `read_write` access to the Linode.  Volumes are automatically detached from deleted Linodes.   <<LB>>  ---   - __CLI__.      ```     linode-cli volumes detach 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     volumes:read_write linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**volume_id** | **i32** | ID of the Volume to detach. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_resize_volume

> models::Volume post_resize_volume(api_version, volume_id, post_resize_volume_request)
Resize a volume

Resize an existing Volume on your Account. In order for this request to complete successfully, your User must have the `read_write` permissions to the Volume.  - Volumes can only be resized up. - Only Volumes with a `status` of \"active\" can be resized.   <<LB>>  ---   - __CLI__.      ```     linode-cli volumes resize 12345 \\   --size 30     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     volumes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**volume_id** | **i32** | ID of the Volume to resize. | [required] |
**post_resize_volume_request** | [**PostResizeVolumeRequest**](PostResizeVolumeRequest.md) | The requested size to increase your Volume to. | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_volume

> models::Volume post_volume(api_version, post_volume_request)
Create a volume

Creates a volume on your account. For this to complete, you need the `add_volumes` grant. Creating a new volume accrues additional charges on your account.   <<LB>>  ---   - __CLI__.      ```     linode-cli volumes create \\   --label my-volume \\   --size 20 \\   --linode_id 12346 \\   --encryption enabled \\   --no-defaults     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     volumes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_volume_request** | [**PostVolumeRequest**](PostVolumeRequest.md) | The requested initial state of a new Volume. | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_volume

> models::Volume put_volume(api_version, volume_id, put_volume_request)
Update a volume

Updates a Volume that you have permission to `read_write`.   <<LB>>  ---   - __CLI__.      ```     linode-cli volumes update 12345 \\   --label my_volume     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     volumes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**volume_id** | **i32** | ID of the Volume to look up. | [required] |
**put_volume_request** | [**PutVolumeRequest**](PutVolumeRequest.md) | If any updated field fails to pass validation, the Volume will not be updated. | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

