# \AclConfigurationsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_object_storage_bucket_acl**](AclConfigurationsApi.md#get_object_storage_bucket_acl) | **GET** /{apiVersion}/object-storage/buckets/{regionId}/{bucket}/object-acl | Get an Object Storage object ACL config
[**put_object_storage_bucket_acl**](AclConfigurationsApi.md#put_object_storage_bucket_acl) | **PUT** /{apiVersion}/object-storage/buckets/{regionId}/{bucket}/object-acl | Update an object's ACL config



## get_object_storage_bucket_acl

> models::GetObjectStorageBucketAcl200Response get_object_storage_bucket_acl(api_version, region_id, bucket, name)
Get an Object Storage object ACL config

View an Object's configured Access Control List (ACL) in this Object Storage bucket. ACLs define who can access your buckets and objects and specify the level of access granted to those users.  > 📘 > > The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/objectops/#get-object-acl) equivalent operation offers more detail.   <<LB>>  ---   - __OAuth scopes__.      ```     object_storage:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**region_id** | **String** | Identifies a region where this bucket lives.  > 📘 > > You can use a `clusterId` in place of `regionId` in requests for buckets that you created using the legacy version of the API. Run [List clusters](https://techdocs.akamai.com/linode-api/reference/get-object-storage-clusters) to see each cluster `id`. | [required] |
**bucket** | **String** | The bucket name. | [required] |
**name** | **String** | The `name` of the object for which to retrieve its Access Control List (ACL). Run the [List Object Storage bucket contents](https://techdocs.akamai.com/linode-api/reference/get-object-storage-bucket-content) operation to access all object names in a bucket. | [required] |

### Return type

[**models::GetObjectStorageBucketAcl200Response**](get_object_storage_bucket_acl_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_object_storage_bucket_acl

> serde_json::Value put_object_storage_bucket_acl(api_version, region_id, bucket, put_object_storage_bucket_acl_request)
Update an object's ACL config

Update an object's configured access control level (ACL) in this Object Storage bucket. ACLs define who can access your buckets and objects, and specify the level of access granted to those users.  > 📘 > > The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/objectops/#set-object-acl) equivalent operation offers more detail.   <<LB>>  ---   - __OAuth scopes__.      ```     object_storage:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**region_id** | **String** | Identifies a region where this bucket lives.  > 📘 > > You can use a `clusterId` in place of `regionId` in requests for buckets that you created using the legacy version of the API. Run [List clusters](https://techdocs.akamai.com/linode-api/reference/get-object-storage-clusters) to see each cluster `id`. | [required] |
**bucket** | **String** | The bucket name. | [required] |
**put_object_storage_bucket_acl_request** | Option<[**PutObjectStorageBucketAclRequest**](PutObjectStorageBucketAclRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

