# \RegionsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_region**](RegionsApi.md#get_region) | **GET** /{apiVersion}/regions/{regionId} | Get a region
[**get_region_availability**](RegionsApi.md#get_region_availability) | **GET** /{apiVersion}/regions/{regionId}/availability | Get a region's availability
[**get_regions**](RegionsApi.md#get_regions) | **GET** /{apiVersion}/regions | List regions
[**get_regions_availability**](RegionsApi.md#get_regions_availability) | **GET** /{apiVersion}/regions/availability | List regions' availability



## get_region

> models::GetRegions200ResponseDataInner get_region(api_version, region_id)
Get a region

Returns a single Region.   <<LB>>  ---   - __CLI__.      ```     linode-cli regions view us-east     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**region_id** | **String** | ID of the Region to look up. | [required] |

### Return type

[**models::GetRegions200ResponseDataInner**](get_regions_200_response_data_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_region_availability

> Vec<models::GetRegionsAvailability200ResponseAllOfDataInner> get_region_availability(api_version, region_id)
Get a region's availability

Returns availability data for a single Region.   <<LB>>  ---   - __CLI__.      ```     linode-cli regions view-avail us-east     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**region_id** | **String** | ID of the Region to look up. | [required] |

### Return type

[**Vec<models::GetRegionsAvailability200ResponseAllOfDataInner>**](get_regions_availability_200_response_allOf_data_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_regions

> models::GetRegions200Response get_regions(api_version)
List regions

Lists the Regions available for Linode services. Not all services are guaranteed to be available in all Regions.   <<LB>>  ---   - __CLI__.      ```     linode-cli regions list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetRegions200Response**](get_regions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_regions_availability

> models::GetRegionsAvailability200Response get_regions_availability(api_version)
List regions' availability

Returns availability data for all Regions.  Currently, this operation returns availability of select premium and GPU plans for select regions.   <<LB>>  ---   - __CLI__.      ```     linode-cli regions list-avail     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |

### Return type

[**models::GetRegionsAvailability200Response**](get_regions_availability_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

