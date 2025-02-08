# \KernelsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_kernel**](KernelsApi.md#get_kernel) | **GET** /{apiVersion}/linode/kernels/{kernelId} | Get a kernel
[**get_kernels**](KernelsApi.md#get_kernels) | **GET** /{apiVersion}/linode/kernels | List kernels



## get_kernel

> models::GetKernels200ResponseDataInner get_kernel(api_version, kernel_id)
Get a kernel

Returns information about a single Kernel.   <<LB>>  ---   - __CLI__.      ```     linode-cli kernels view latest-64bit     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**kernel_id** | **String** | ID of the Kernel to look up. | [required] |

### Return type

[**models::GetKernels200ResponseDataInner**](get_kernels_200_response_data_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_kernels

> models::GetKernels200Response get_kernels(api_version, page, page_size)
List kernels

Lists available Kernels.  Due to the extensive list of available kernels, please keep [pagination](https://techdocs.akamai.com/linode-api/reference/pagination) controls in mind when managing responses to this operation.   <<LB>>  ---   - __CLI__.      ```     linode-cli kernels list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetKernels200Response**](get_kernels_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

