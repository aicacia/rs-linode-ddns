# \KeysApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_object_storage_key**](KeysApi.md#delete_object_storage_key) | **DELETE** /{apiVersion}/object-storage/keys/{keyId} | Revoke an Object Storage key
[**get_object_storage_key**](KeysApi.md#get_object_storage_key) | **GET** /{apiVersion}/object-storage/keys/{keyId} | Get an Object Storage key
[**get_object_storage_keys**](KeysApi.md#get_object_storage_keys) | **GET** /{apiVersion}/object-storage/keys | List Object Storage keys
[**post_object_storage_keys**](KeysApi.md#post_object_storage_keys) | **POST** /{apiVersion}/object-storage/keys | Create an Object Storage key
[**put_object_storage_key**](KeysApi.md#put_object_storage_key) | **PUT** /{apiVersion}/object-storage/keys/{keyId} | Update an Object Storage key



## delete_object_storage_key

> serde_json::Value delete_object_storage_key(api_version, key_id)
Revoke an Object Storage key

Revokes an Object Storage Key. This keypair will no longer be usable by third-party clients.  - A successful request triggers an `obj_access_key_delete` event.   <<LB>>  ---   - __CLI__.      ```     linode-cli object-storage keys-delete 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     object_storage:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**key_id** | **i32** | The key to look up. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_storage_key

> models::GetObjectStorageKeys200ResponseDataInner get_object_storage_key(api_version, key_id)
Get an Object Storage key

Returns a single Object Storage key provisioned for your account.   <<LB>>  ---   - __CLI__.      ```     linode-cli object-storage keys-view \\   --keyId 12345     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     object_storage:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**key_id** | **i32** | The key to look up. | [required] |

### Return type

[**models::GetObjectStorageKeys200ResponseDataInner**](get_object_storage_keys_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_storage_keys

> models::GetObjectStorageKeys200Response get_object_storage_keys(api_version)
List Object Storage keys

Returns a paginated list of Object Storage keys for authenticating to the Object Storage S3 API.   <<LB>>  ---   - __CLI__.      ```     linode-cli object-storage keys-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     object_storage:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetObjectStorageKeys200Response**](get_object_storage_keys_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_storage_keys

> models::GetObjectStorageKeys200ResponseDataInner post_object_storage_keys(api_version, post_object_storage_keys_request)
Create an Object Storage key

Provisions a new Object Storage key for authenticating to the Object Storage S3 API. A successful request triggers an `obj_access_key_create` [event](https://techdocs.akamai.com/linode-api/reference/get-events).  > 📘 > > Accounts with negative balances can't access this operation.  **The `regions` and `region` parameters**  When creating an Object Storage key, specify one or more data centers ([regions](https://techdocs.akamai.com/linode-api/reference/get-regions)) where you want to create and manage Object Storage buckets.  - **The `regions` array**. Populate it with `regionId` values. The resulting Object Storage key grants access to list and create new buckets in these regions. This *doesn't* give access to manage content in these buckets. To address this, you can:    - Use the `bucket_access` array instead to grant management access, per bucket.    - Use [bucket policies](https://www.linode.com/docs/products/storage/object-storage/guides/bucket-policies/) to change the access for this key.  - **The `bucket_access` array**. This optional array lets you set up limited keys. Include individual objects naming a `regionId`, the target `bucket_name`, and the `permissions` for the Object Storage key. Use the resulting key to manage content in the `bucket_name`, based on the permission level set. You can also use the key to create new buckets in the named region. However, the key doesn't have access to manage content in the newly created bucket. You can grant it this access using [bucket policies](https://www.linode.com/docs/products/storage/object-storage/guides/bucket-policies/).  - **Combine the two to apply varying levels of access in the key**. For example, set `regions` to `us-west` to give the key bucket list and create access in that region. Then, set up the `bucket_access` array to give access to a specific `bucket_name` in the `us-east` region. The key has access to manage content in that `bucket_name` and list and create buckets in the `us-east` region, too. If you include the same region in both, the settings applied in the `bucket_access` array take precedence. For example, assume you include `us-east` in the `regions` array, expecting to only give bucket list and creation access to that region. If you also set `us-east` as a `region` in the `bucket_access` array, the Object Storage key gives access to manage content in the specified `bucket_name`, and lets you list and create buckets in that region.  **The `cluster` parameter (legacy)**  For backward compatibility, include the `cluster` parameter to create an Object Storage key. Use the `clusterId` equivalent (us-west-1) instead of the `regionId` (us-west). Leave the `regions` array out. If including the `bucket_access` array to limit access, omit `region` from each object. Use the resulting key in clusters in all supported regions.  > 📘 > > While the API supports this method, you should use the `regions` parameters, instead.  - **Unlimited access**. Omit the `bucket_access` array. The Object Storage key has unlimited cluster access to all buckets, with all permissions.  - **Limited access**. Include the `bucket_access` array. Set the target `bucket_name` and the level of `permissions` for access to that bucket. Use the resulting key to manage content in the named bucket. A limited Object Storage key can [list all buckets](https://techdocs.akamai.com/linode-api/reference/get-object-storage-buckets) and [create a new bucket](https://techdocs.akamai.com/linode-api/reference/post-object-storage-bucket). However, you can't use the key to perform any actions on a bucket, unless the key has access to it. You can use [bucket policies](https://www.linode.com/docs/products/storage/object-storage/guides/bucket-policies/) to modify a key's access.   <<LB>>  ---   - __CLI__.      ```     linode-cli object-storage keys-create \\   --label \"my-object-storage-key\" \\   --bucket_access '[{\"region\": \"ap-south\", \"bucket_name\": \"bucket-example-1\", \"permissions\": \"read_write\" }]'     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     object_storage:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_object_storage_keys_request** | Option<[**PostObjectStorageKeysRequest**](PostObjectStorageKeysRequest.md)> | The settings necessary to create a new key. |  |

### Return type

[**models::GetObjectStorageKeys200ResponseDataInner**](get_object_storage_keys_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_object_storage_key

> models::PutObjectStorageKey200Response put_object_storage_key(api_version, key_id, put_object_storage_key_request)
Update an Object Storage key

Updates an Object Storage key on your account. A successful request triggers an `obj_access_key_update` [event](https://techdocs.akamai.com/linode-api/reference/get-events).   <<LB>>  ---   - __CLI__.      ```     linode-cli object-storage keys-update \\   --keyId 12345   --label \"my-object-storage-key\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     object_storage:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**key_id** | **i32** | The key to look up. | [required] |
**put_object_storage_key_request** | Option<[**PutObjectStorageKeyRequest**](PutObjectStorageKeyRequest.md)> | The fields to update. |  |

### Return type

[**models::PutObjectStorageKey200Response**](put_object_storage_key_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

