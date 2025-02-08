# \BucketsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_object_storage_bucket**](BucketsApi.md#delete_object_storage_bucket) | **DELETE** /{apiVersion}/object-storage/buckets/{regionId}/{bucket} | Remove an Object Storage bucket
[**get_object_storage_bucket**](BucketsApi.md#get_object_storage_bucket) | **GET** /{apiVersion}/object-storage/buckets/{regionId}/{bucket} | Get an Object Storage bucket
[**get_object_storage_bucketin_cluster**](BucketsApi.md#get_object_storage_bucketin_cluster) | **GET** /{apiVersion}/object-storage/buckets/{regionId} | List Object Storage buckets per region
[**get_object_storage_buckets**](BucketsApi.md#get_object_storage_buckets) | **GET** /{apiVersion}/object-storage/buckets | List Object Storage buckets
[**post_object_storage_bucket**](BucketsApi.md#post_object_storage_bucket) | **POST** /{apiVersion}/object-storage/buckets | Create an Object Storage bucket
[**post_object_storage_object_url**](BucketsApi.md#post_object_storage_object_url) | **POST** /{apiVersion}/object-storage/buckets/{regionId}/{bucket}/object-url | Create a URL for an object



## delete_object_storage_bucket

> serde_json::Value delete_object_storage_bucket(api_version, region_id, bucket)
Remove an Object Storage bucket

Removes a single bucket.  > 📘 > > - You need to remove all objects from a bucket before you can delete it. While you *can* delete a bucket using the [s3cmd command-line tool](https://www.linode.com/docs/products/storage/object-storage/guides/s3cmd/#delete-a-bucket), this operation fails if the bucket contains too many objects. The best way to empty large buckets is to use the [S3 API to configure lifecycle policies](https://docs.ceph.com/en/latest/radosgw/bucketpolicy/#). Set a policy to remove all objects, wait a day or more for the system to remove all objects, then delete the bucket. > > - The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/bucketops/#delete-bucket) equivalent operation offers more detail.   <<LB>>  ---   - __OAuth scopes__.      ```     object_storage:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**region_id** | **String** | Identifies a region where this bucket lives.  > 📘 > > You can use a `clusterId` in place of `regionId` in requests for buckets that you created using the legacy version of the API. Run [List clusters](https://techdocs.akamai.com/linode-api/reference/get-object-storage-clusters) to see each cluster `id`. | [required] |
**bucket** | **String** | The bucket name. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_storage_bucket

> models::GetObjectStorageBuckets200ResponseDataInner get_object_storage_bucket(api_version, region_id, bucket)
Get an Object Storage bucket

Returns a single Object Storage bucket.  > 📘 > > The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/bucketops/#get-bucket) equivalent operation offers more detail.   <<LB>>  ---   - __OAuth scopes__.      ```     object_storage:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**region_id** | **String** | Identifies a region where this bucket lives.  > 📘 > > You can use a `clusterId` in place of `regionId` in requests for buckets that you created using the legacy version of the API. Run [List clusters](https://techdocs.akamai.com/linode-api/reference/get-object-storage-clusters) to see each cluster `id`. | [required] |
**bucket** | **String** | The bucket name. | [required] |

### Return type

[**models::GetObjectStorageBuckets200ResponseDataInner**](get_object_storage_buckets_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_storage_bucketin_cluster

> models::GetObjectStorageBucketinCluster200Response get_object_storage_bucketin_cluster(api_version, region_id)
List Object Storage buckets per region

Returns a list of buckets on your account, in the specified region.  > 📘 > > The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/bucketops/#get-bucket) equivalent operation offers more detail.   <<LB>>  ---   - __OAuth scopes__.      ```     object_storage:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**region_id** | **String** | Identifies a region where this bucket lives.  > 📘 > > You can use a `clusterId` in place of `regionId` in requests for buckets that you created using the legacy version of the API. Run [List clusters](https://techdocs.akamai.com/linode-api/reference/get-object-storage-clusters) to see each cluster `id`. | [required] |

### Return type

[**models::GetObjectStorageBucketinCluster200Response**](get_object_storage_bucketin_cluster_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_storage_buckets

> models::GetObjectStorageBuckets200Response get_object_storage_buckets(api_version)
List Object Storage buckets

Returns a paginated list of all Object Storage buckets available in your account.  > 📘 > > The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/serviceops/#list-buckets) equivalent operation offers more detail.   <<LB>>  ---   - __OAuth scopes__.      ```     object_storage:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetObjectStorageBuckets200Response**](get_object_storage_buckets_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_storage_bucket

> models::GetObjectStorageBuckets200ResponseDataInner post_object_storage_bucket(api_version, post_object_storage_bucket_request)
Create an Object Storage bucket

Creates an Object Storage bucket in the specified data center ([region](https://techdocs.akamai.com/linode-api/reference/get-regions)). If the bucket already exists on your account, this operation returns a 200 response with that bucket as if the API just created it.  > 📘 > > - Accounts with negative balances can't access this operation. > > - The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/bucketops/#put-bucket) equivalent operation offers more detail. > > - The API still supports the `clusterId` equivalent (`us-west-1`) when setting a `region` for a new bucket, but you should use the `regionId` (`us-west`), instead.   <<LB>>  ---   - __OAuth scopes__.      ```     object_storage:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_object_storage_bucket_request** | Option<[**PostObjectStorageBucketRequest**](PostObjectStorageBucketRequest.md)> | Information about the bucket you want to create. |  |

### Return type

[**models::GetObjectStorageBuckets200ResponseDataInner**](get_object_storage_buckets_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_storage_object_url

> models::PostObjectStorageObjectUrl200Response post_object_storage_object_url(api_version, region_id, bucket, post_object_storage_object_url_request)
Create a URL for an object

Creates a pre-signed URL to access a single object in a bucket. Use it to share, create, or delete objects by using the appropriate HTTP method in your request body's `method` parameter.  > 📘 > > The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/) equivalent operation offers more detail.   <<LB>>  ---   - __OAuth scopes__.      ```     object_storage:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**region_id** | **String** | Identifies a region where this bucket lives.  > 📘 > > You can use a `clusterId` in place of `regionId` in requests for buckets that you created using the legacy version of the API. Run [List clusters](https://techdocs.akamai.com/linode-api/reference/get-object-storage-clusters) to see each cluster `id`. | [required] |
**bucket** | **String** | The bucket name. | [required] |
**post_object_storage_object_url_request** | Option<[**PostObjectStorageObjectUrlRequest**](PostObjectStorageObjectUrlRequest.md)> | Information about the request to sign. |  |

### Return type

[**models::PostObjectStorageObjectUrl200Response**](post_object_storage_object_url_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

