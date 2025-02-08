# \TlssslCertificatesApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_object_storage_ssl**](TlssslCertificatesApi.md#delete_object_storage_ssl) | **DELETE** /{apiVersion}/object-storage/buckets/{regionId}/{bucket}/ssl | Delete an Object Storage TLS/SSL certificate
[**get_object_storage_ssl**](TlssslCertificatesApi.md#get_object_storage_ssl) | **GET** /{apiVersion}/object-storage/buckets/{regionId}/{bucket}/ssl | Get an Object Storage TLS/SSL certificate
[**post_object_storage_ssl**](TlssslCertificatesApi.md#post_object_storage_ssl) | **POST** /{apiVersion}/object-storage/buckets/{regionId}/{bucket}/ssl | Upload an Object Storage TLS/SSL certificate



## delete_object_storage_ssl

> serde_json::Value delete_object_storage_ssl(api_version, region_id, bucket)
Delete an Object Storage TLS/SSL certificate

Deletes this Object Storage bucket's user uploaded TLS/SSL certificate and private key.   <<LB>>  ---   - __CLI__.      ```     linode-cli object-storage ssl-delete \\   us-east-1 example-bucket     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     object_storage:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

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


## get_object_storage_ssl

> models::GetObjectStorageSsl200Response get_object_storage_ssl(api_version, region_id, bucket)
Get an Object Storage TLS/SSL certificate

Returns a boolean value indicating if this bucket has a corresponding TLS/SSL certificate that was uploaded by an Account user.   <<LB>>  ---   - __CLI__.      ```     linode-cli object-storage ssl-view \\   us-east-1 example-bucket     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     object_storage:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**region_id** | **String** | Identifies a region where this bucket lives.  > 📘 > > You can use a `clusterId` in place of `regionId` in requests for buckets that you created using the legacy version of the API. Run [List clusters](https://techdocs.akamai.com/linode-api/reference/get-object-storage-clusters) to see each cluster `id`. | [required] |
**bucket** | **String** | The bucket name. | [required] |

### Return type

[**models::GetObjectStorageSsl200Response**](get_object_storage_ssl_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_storage_ssl

> models::GetObjectStorageSsl200Response post_object_storage_ssl(api_version, region_id, bucket, post_object_storage_ssl_request)
Upload an Object Storage TLS/SSL certificate

Upload a TLS/SSL certificate and private key to be served when you visit your Object Storage bucket via HTTPS. Your TLS/SSL certificate and private key are stored encrypted at rest.  To replace an expired certificate, [delete your current certificates](https://techdocs.akamai.com/linode-api/reference/delete-object-storage-ssl) and upload a new one.   <<LB>>  ---   - __CLI__.      ```     linode-cli object-storage ssl-upload \\   us-east-1 example-bucket \\   --certificate \"-----BEGIN CERTIFICATE-----                  CERTIFICATE_INFORMATION                  -----END CERTIFICATE-----\" \\   --private_key \"-----BEGIN PRIVATE KEY-----                  PRIVATE_KEY_INFORMATION                  -----END PRIVATE KEY-----\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     object_storage:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**region_id** | **String** | Identifies a region where this bucket lives.  > 📘 > > You can use a `clusterId` in place of `regionId` in requests for buckets that you created using the legacy version of the API. Run [List clusters](https://techdocs.akamai.com/linode-api/reference/get-object-storage-clusters) to see each cluster `id`. | [required] |
**bucket** | **String** | The bucket name. | [required] |
**post_object_storage_ssl_request** | Option<[**PostObjectStorageSslRequest**](PostObjectStorageSslRequest.md)> | Upload this TLS/SSL certificate with its corresponding secret key. |  |

### Return type

[**models::GetObjectStorageSsl200Response**](get_object_storage_ssl_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

