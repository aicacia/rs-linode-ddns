# \LkeVersionsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lke_version**](LkeVersionsApi.md#get_lke_version) | **GET** /{apiVersion}/lke/versions/{version} | Get a Kubernetes version
[**get_lke_versions**](LkeVersionsApi.md#get_lke_versions) | **GET** /{apiVersion}/lke/versions | List Kubernetes versions



## get_lke_version

> models::GetLkeVersions200ResponseDataInner get_lke_version(api_version, version)
Get a Kubernetes version

View a Kubernetes version available for deployment to a Kubernetes cluster.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke version-view 1.31     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**version** | **String** | The LKE version to view. | [required] |

### Return type

[**models::GetLkeVersions200ResponseDataInner**](get_lke_versions_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lke_versions

> models::GetLkeVersions200Response get_lke_versions(api_version)
List Kubernetes versions

List the Kubernetes versions available for deployment to a Kubernetes cluster.   <<LB>>  ---   - __CLI__.      ```     linode-cli lke versions-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     lke:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetLkeVersions200Response**](get_lke_versions_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

