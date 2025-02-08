# \BucketContentsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_object_storage_bucket_content**](BucketContentsApi.md#get_object_storage_bucket_content) | **GET** /{apiVersion}/object-storage/buckets/{regionId}/{bucket}/object-list | List Object Storage bucket contents



## get_object_storage_bucket_content

> models::GetObjectStorageBucketContent200Response get_object_storage_bucket_content(api_version, region_id, bucket, marker, delimiter, prefix, page_size)
List Object Storage bucket contents

Returns the contents of a bucket. The contents are paginated using a `marker`, that's the name of the last object on the previous page.  Objects can also be filtered by `prefix` and `delimiter`. See [Filtering and sorting](https://techdocs.akamai.com/linode-api/reference/filtering-and-sorting) for more information.  > 📘 > > The [S3 API](https://docs.ceph.com/en/latest/radosgw/s3/objectops/#get-object) equivalent operation offers more detail.   <<LB>>  ---   - __OAuth scopes__.      ```     object_storage:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**region_id** | **String** | Identifies a region where this bucket lives.  > 📘 > > You can use a `clusterId` in place of `regionId` in requests for buckets that you created using the legacy version of the API. Run [List clusters](https://techdocs.akamai.com/linode-api/reference/get-object-storage-clusters) to see each cluster `id`. | [required] |
**bucket** | **String** | The bucket name. | [required] |
**marker** | Option<**String**> | The \"marker\" for this request, which can be used to paginate through large buckets. Its value should be the value of the `next_marker` property returned with the last page. Listing bucket contents _does not_ support arbitrary page access. See the `next_marker` property in the responses section for more details. |  |
**delimiter** | Option<**String**> | The delimiter for object names; if given, object names will be returned up to the first occurrence of this character. This is most commonly used with the `/` character to allow bucket transversal in a manner similar to a filesystem, however any delimiter may be used. Use in conjunction with `prefix` to see object names past the first occurrence of the delimiter. |  |
**prefix** | Option<**String**> | Filters objects returned to only those whose name start with the given prefix. Commonly used in conjunction with `delimiter` to allow transversal of bucket contents in a manner similar to a filesystem. |  |
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetObjectStorageBucketContent200Response**](get_object_storage_bucket_content_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

