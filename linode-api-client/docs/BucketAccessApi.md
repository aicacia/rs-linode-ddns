# \BucketAccessApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_object_storage_bucket_access**](BucketAccessApi.md#post_object_storage_bucket_access) | **POST** /{apiVersion}/object-storage/buckets/{regionId}/{bucket}/access | Modify access to an Object Storage bucket
[**put_storage_bucket_access**](BucketAccessApi.md#put_storage_bucket_access) | **PUT** /{apiVersion}/object-storage/buckets/{regionId}/{bucket}/access | Update access to an Object Storage bucket



## post_object_storage_bucket_access

> serde_json::Value post_object_storage_bucket_access(api_version, region_id, bucket, post_object_storage_bucket_access_request)
Modify access to an Object Storage bucket

Apply basic Cross-origin Resource Sharing (CORS) and Access Control Level (ACL) settings. You can configure CORS for all origins and set canned ACL settings.  > 📘 > > For more fine-grained control of both systems, use the [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/bucketops/#put-bucket-acl).   <<LB>>  ---   - __OAuth scopes__.      ```     object_storage:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**region_id** | **String** | Identifies a region where this bucket lives.  > 📘 > > You can use a `clusterId` in place of `regionId` in requests for buckets that you created using the legacy version of the API. Run [List clusters](https://techdocs.akamai.com/linode-api/reference/get-object-storage-clusters) to see each cluster `id`. | [required] |
**bucket** | **String** | The bucket name. | [required] |
**post_object_storage_bucket_access_request** | Option<[**PostObjectStorageBucketAccessRequest**](PostObjectStorageBucketAccessRequest.md)> | The changes to make to the bucket's access controls. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_storage_bucket_access

> serde_json::Value put_storage_bucket_access(api_version, region_id, bucket, put_storage_bucket_access_request)
Update access to an Object Storage bucket

Update basic Cross-origin Resource Sharing (CORS) and Access Control Level (ACL) settings. You can configure CORS for all origins and set canned ACL settings.  > 📘 > > For more fine-grained control of both systems, use the [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/bucketops/#put-bucket-acl).   <<LB>>  ---   - __OAuth scopes__.      ```     object_storage:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**region_id** | **String** | Identifies a region where this bucket lives.  > 📘 > > You can use a `clusterId` in place of `regionId` in requests for buckets that you created using the legacy version of the API. Run [List clusters](https://techdocs.akamai.com/linode-api/reference/get-object-storage-clusters) to see each cluster `id`. | [required] |
**bucket** | **String** | The bucket name. | [required] |
**put_storage_bucket_access_request** | Option<[**PutStorageBucketAccessRequest**](PutStorageBucketAccessRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

