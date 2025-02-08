# \VpcsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_vpc**](VpcsApi.md#delete_vpc) | **DELETE** /{apiVersion}/vpcs/{vpcId} | Delete a VPC
[**get_vpc**](VpcsApi.md#get_vpc) | **GET** /{apiVersion}/vpcs/{vpcId} | Get a VPC
[**get_vpcs**](VpcsApi.md#get_vpcs) | **GET** /{apiVersion}/vpcs | List VPCs
[**post_vpc**](VpcsApi.md#post_vpc) | **POST** /{apiVersion}/vpcs | Create a VPC
[**put_vpc**](VpcsApi.md#put_vpc) | **PUT** /{apiVersion}/vpcs/{vpcId} | Update a VPC



## delete_vpc

> serde_json::Value delete_vpc(api_version, vpc_id)
Delete a VPC

Delete a single VPC and all of its Subnets.  - The User accessing this operation must have `read_write` grants to the VPC. - A successful request triggers a `vpc_delete` event and `subnet_delete` events for each deleted VPC Subnet. - All of the VPC's Subnets must be eligible for deletion. Accordingly, all Configuration Profile Interfaces that each Subnet is assigned to must first be deleted. If those Interfaces are active, the associated Linodes must first be shut down before they can be removed. If any Subnet cannot be deleted, then neither the VPC nor any of its Subnets are deleted.   <<LB>>  ---   - __CLI__.      ```     linode-cli vpcs delete $vpcId     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     vpc:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**vpc_id** | **i32** | The `id` of the VPC. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vpc

> models::GetVpc200Response get_vpc(api_version, vpc_id)
Get a VPC

Get information about a single VPC.   <<LB>>  ---   - __CLI__.      ```     linode-cli vpcs view $vpcId     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**vpc_id** | **i32** | The `id` of the VPC. | [required] |

### Return type

[**models::GetVpc200Response**](get_vpc_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vpcs

> models::GetVpcs200Response get_vpcs(api_version, page, page_size)
List VPCs

Display all VPCs on your account.   <<LB>>  ---   - __CLI__.      ```     linode-cli vpcs list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetVpcs200Response**](get_vpcs_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_vpc

> models::PostVpc200Response post_vpc(api_version, post_vpc_request)
Create a VPC

Create a new VPC and optionally associated VPC Subnets.  - Users must have the `add_vpc` grant to access this operation. - A successful request triggers a `vpc_create` event and `subnet_create` events for any created VPC Subnets.  Once a VPC is created, it can be attached to a Linode by assigning a VPC Subnet to one of the Linode's Configuration Profile Interfaces. This step can be accomplished with the following operations:  - [Create a Linode](https://techdocs.akamai.com/linode-api/reference/post-linode-instance) - [Create a config profile](https://techdocs.akamai.com/linode-api/reference/post-add-linode-config) - [Update a config profile](https://techdocs.akamai.com/linode-api/reference/put-linode-config) - [Add a configuration profile interface](https://techdocs.akamai.com/linode-api/reference/post-linode-config-interface)   <<LB>>  ---   - __CLI__.      ```     linode-cli vpcs create \\   --description \"A description of my VPC.\" \\   --label cool-vpc \\   --region us-east \\   --subnets.label cool-vpc-subnet \\   --subnets.ipv4 10.0.1.0/24     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     vpc:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_vpc_request** | [**PostVpcRequest**](PostVpcRequest.md) | VPC Create request object. | [required] |

### Return type

[**models::PostVpc200Response**](post_vpc_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_vpc

> models::PutVpc200Response put_vpc(api_version, vpc_id, put_vpc_request)
Update a VPC

Update an existing VPC.  - The User accessing this operation must have `read_write` grants to the VPC. - A successful request triggers a `vpc_update` event.  To update a VPC's Subnet, run the [Update a VPC subnet](https://techdocs.akamai.com/linode-api/reference/put-vpc-subnet) operation.   <<LB>>  ---   - __CLI__.      ```     linode-cli vpcs update $vpcId \\   --description \"A description of my VPC.\"   --label cool-vpc     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     vpc:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**vpc_id** | **i32** | The `id` of the VPC. | [required] |
**put_vpc_request** | [**PutVpcRequest**](PutVpcRequest.md) | VPC Update request object. | [required] |

### Return type

[**models::PutVpc200Response**](put_vpc_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

