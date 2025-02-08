# \BetaProgramsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_beta_program**](BetaProgramsApi.md#get_beta_program) | **GET** /{apiVersion}/betas/{betaId} | Get a Beta program
[**get_beta_programs**](BetaProgramsApi.md#get_beta_programs) | **GET** /{apiVersion}/betas | List Beta programs
[**get_enrolled_beta_program**](BetaProgramsApi.md#get_enrolled_beta_program) | **GET** /{apiVersion}/account/betas/{betaId} | Get an enrolled Beta program
[**get_enrolled_beta_programs**](BetaProgramsApi.md#get_enrolled_beta_programs) | **GET** /{apiVersion}/account/betas | List enrolled Beta programs
[**post_beta_program**](BetaProgramsApi.md#post_beta_program) | **POST** /{apiVersion}/account/betas | Enroll in a Beta program



## get_beta_program

> models::GetBetaPrograms200ResponseAllOfDataInner get_beta_program(api_version, beta_id)
Get a Beta program

Display information about a Beta Program. This operation can be used to access inactive as well as active Beta Programs.  Only unrestricted Users can access this operation.   <<LB>>  ---   - __CLI__.      ```     linode-cli betas view $betaId     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**beta_id** | **String** | The ID of the Beta Program. | [required] |

### Return type

[**models::GetBetaPrograms200ResponseAllOfDataInner**](get_beta_programs_200_response_allOf_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_beta_programs

> models::GetBetaPrograms200Response get_beta_programs(api_version, page, page_size)
List Beta programs

Display all active Beta Programs.  Only unrestricted Users can access this operation.   <<LB>>  ---   - __CLI__.      ```     linode-cli betas list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetBetaPrograms200Response**](get_beta_programs_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_enrolled_beta_program

> models::GetEnrolledBetaPrograms200ResponseAllOfDataInner get_enrolled_beta_program(api_version, beta_id)
Get an enrolled Beta program

Display an enrolled Beta Program for your Account. The Beta Program may be inactive.  Only unrestricted Users can access this operation.   <<LB>>  ---   - __CLI__.      ```     linode-cli betas enrolled-view $betaId     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**beta_id** | **String** | The ID of the Beta Program. | [required] |

### Return type

[**models::GetEnrolledBetaPrograms200ResponseAllOfDataInner**](get_enrolled_beta_programs_200_response_allOf_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_enrolled_beta_programs

> models::GetEnrolledBetaPrograms200Response get_enrolled_beta_programs(api_version, page, page_size)
List enrolled Beta programs

Display all enrolled Beta Programs for your Account. Includes inactive as well as active Beta Programs.  Only unrestricted Users can access this operation.   <<LB>>  ---   - __CLI__.      ```     linode-cli betas enrolled     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetEnrolledBetaPrograms200Response**](get_enrolled_beta_programs_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_beta_program

> serde_json::Value post_beta_program(api_version, post_beta_program_request)
Enroll in a Beta program

Enroll your Account in an active Beta Program.  Only unrestricted Users can access this operation.  To view active Beta Programs, run the [List beta programs](https://techdocs.akamai.com/linode-api/reference/get-beta-programs) operation.  Active Beta Programs may have a limited number of enrollments. If a Beta Program has reached is maximum number of enrollments, an error is returned even though the request is successful.  Beta Programs with `\"greenlight_only\": true` can only be enrolled by Accounts that participate in the [Greenlight](https://www.linode.com/green-light/) program.   <<LB>>  ---   - __CLI__.      ```     linode-cli betas enroll --id example_open     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_beta_program_request** | [**PostBetaProgramRequest**](PostBetaProgramRequest.md) | Updated information for the Managed MySQL Database. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

