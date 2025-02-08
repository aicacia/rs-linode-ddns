# \VpcSubnetsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_vpc_subnet**](VpcSubnetsApi.md#delete_vpc_subnet) | **DELETE** /{apiVersion}/vpcs/{vpcId}/subnets/{vpcSubnetId} | Delete a VPC subnet
[**get_vpc_subnet**](VpcSubnetsApi.md#get_vpc_subnet) | **GET** /{apiVersion}/vpcs/{vpcId}/subnets/{vpcSubnetId} | Get a VPC subnet
[**get_vpc_subnets**](VpcSubnetsApi.md#get_vpc_subnets) | **GET** /{apiVersion}/vpcs/{vpcId}/subnets | List VPC subnets
[**post_vpc_subnet**](VpcSubnetsApi.md#post_vpc_subnet) | **POST** /{apiVersion}/vpcs/{vpcId}/subnets | Create a VPC subnet
[**put_vpc_subnet**](VpcSubnetsApi.md#put_vpc_subnet) | **PUT** /{apiVersion}/vpcs/{vpcId}/subnets/{vpcSubnetId} | Update a VPC subnet



## delete_vpc_subnet

> serde_json::Value delete_vpc_subnet(api_version, vpc_id, vpc_subnet_id)
Delete a VPC subnet

Delete a single VPC Subnet.  The user accessing this operation must have `read_write` grants to the VPC. A successful request triggers a `subnet_delete` event.  __Note__. You need to delete all the Configuration Profile Interfaces that this Subnet is assigned to before you can delete it. If those Interfaces are active, the associated Linode needs to be shut down before they can be removed.   <<LB>>  ---   - __CLI__.      ```     linode-cli vpcs subnet-delete $vpcId $vpcSubnetId     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     vpc:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**vpc_id** | **i32** | The `id` of the VPC. | [required] |
**vpc_subnet_id** | **i32** | The `id` of the VPC Subnet. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vpc_subnet

> models::GetVpcSubnet200Response get_vpc_subnet(api_version, vpc_id, vpc_subnet_id)
Get a VPC subnet

Get information about a single VPC Subnet.   <<LB>>  ---   - __CLI__.      ```     linode-cli vpcs subnet-view $vpcId $vpcSubnetId     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**vpc_id** | **i32** | The `id` of the VPC. | [required] |
**vpc_subnet_id** | **i32** | The `id` of the VPC Subnet. | [required] |

### Return type

[**models::GetVpcSubnet200Response**](get_vpc_subnet_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vpc_subnets

> models::GetVpcSubnets200Response get_vpc_subnets(api_version, vpc_id, page, page_size)
List VPC subnets

Get information about all VPC Subnets associated with a VPC.   <<LB>>  ---   - __CLI__.      ```     linode-cli vpcs subnets-list $vpcId     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**vpc_id** | **i32** | The `id` of the VPC. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetVpcSubnets200Response**](get_vpc_subnets_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_vpc_subnet

> models::PostVpcSubnet200Response post_vpc_subnet(api_version, vpc_id, post_vpc_subnet_request)
Create a VPC subnet

Create a VPC Subnet.  - The User accessing this operation must have `read_write` grants to the VPC. - A successful request triggers a `subnet_create` event.  Once a VPC Subnet is created, it can be attached to a Linode by assigning the Subnet to one of the Linode's Configuration Profile Interfaces. This step can be accomplished with the following operations:  - [Create a Linode](https://techdocs.akamai.com/linode-api/reference/post-linode-instance) - [Create a config profile](https://techdocs.akamai.com/linode-api/reference/post-add-linode-config) - [Update a config profile](https://techdocs.akamai.com/linode-api/reference/put-linode-config) - [Add a configuration profile interface](https://techdocs.akamai.com/linode-api/reference/post-linode-config-interface)   <<LB>>  ---   - __CLI__.      ```     linode-cli vpcs subnet-create $vpcId \\   --label cool-vpc-subnet \\   --ipv4 10.0.1.0/24     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     vpc:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**vpc_id** | **i32** | The `id` of the VPC. | [required] |
**post_vpc_subnet_request** | [**PostVpcSubnetRequest**](PostVpcSubnetRequest.md) | VPC Subnet Create request object. | [required] |

### Return type

[**models::PostVpcSubnet200Response**](post_vpc_subnet_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_vpc_subnet

> models::PutVpcSubnet200Response put_vpc_subnet(api_version, vpc_id, vpc_subnet_id, put_vpc_subnet_request)
Update a VPC subnet

Update a VPC Subnet.  - The User accessing this operation must have `read_write` grants to the VPC. - A successful request triggers a `subnet_update` event.   <<LB>>  ---   - __CLI__.      ```     linode-cli vpcs subnet-update $vpcId \\   --label cool-vpc-subnet     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     vpc:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**vpc_id** | **i32** | The `id` of the VPC. | [required] |
**vpc_subnet_id** | **i32** | The `id` of the VPC Subnet. | [required] |
**put_vpc_subnet_request** | [**PutVpcSubnetRequest**](PutVpcSubnetRequest.md) | VPC Update request object. | [required] |

### Return type

[**models::PutVpcSubnet200Response**](put_vpc_subnet_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

