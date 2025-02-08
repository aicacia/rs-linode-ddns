# \ServiceTransfersApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_service_transfer**](ServiceTransfersApi.md#delete_service_transfer) | **DELETE** /{apiVersion}/account/service-transfers/{token} | Cancel a service transfer
[**get_service_transfer**](ServiceTransfersApi.md#get_service_transfer) | **GET** /{apiVersion}/account/service-transfers/{token} | Get a service transfer request
[**get_service_transfers**](ServiceTransfersApi.md#get_service_transfers) | **GET** /{apiVersion}/account/service-transfers | List service transfers
[**post_accept_service_transfer**](ServiceTransfersApi.md#post_accept_service_transfer) | **POST** /{apiVersion}/account/service-transfers/{token}/accept | Accept a service transfer
[**post_service_transfer**](ServiceTransfersApi.md#post_service_transfer) | **POST** /{apiVersion}/account/service-transfers | Request a service transfer



## delete_service_transfer

> serde_json::Value delete_service_transfer(api_version, token)
Cancel a service transfer

Cancels the Service Transfer for the provided token. Once canceled, a transfer cannot be accepted or otherwise acted on in any way. If canceled in error, the transfer must be [created](https://techdocs.akamai.com/linode-api/reference/post-service-transfer) again.  When canceled, an email notification for the cancellation is sent to the account that created this transfer. Transfers can not be canceled if they are expired or have been accepted.  This operation can only be accessed by the unrestricted users of the account that created this transfer.   <<LB>>  ---   - __CLI__.      ```     linode-cli service-transfers \\   cancel 123E4567-E89B-12D3-A456-426614174000     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**token** | **uuid::Uuid** | The UUID of the Service Transfer. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_transfer

> models::GetServiceTransfers200ResponseDataInner get_service_transfer(api_version, token)
Get a service transfer request

Returns the details of the Service Transfer for the provided token.  While a transfer is pending, any unrestricted user _of any account_ can access this operation. After a transfer has been accepted, it can only be viewed by unrestricted users of the accounts that created and accepted the transfer. If canceled or expired, only unrestricted users of the account that created the transfer can view it.   <<LB>>  ---   - __CLI__.      ```     linode-cli service-transfers \\   view 123E4567-E89B-12D3-A456-426614174000     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**token** | **uuid::Uuid** | The UUID of the Service Transfer. | [required] |

### Return type

[**models::GetServiceTransfers200ResponseDataInner**](get_service_transfers_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_transfers

> models::GetServiceTransfers200Response get_service_transfers(api_version, page, page_size)
List service transfers

Returns a collection of all created and accepted Service Transfers for this account, regardless of the user that created or accepted the transfer.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli service-transfers \\   list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetServiceTransfers200Response**](get_service_transfers_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_accept_service_transfer

> serde_json::Value post_accept_service_transfer(api_version, token)
Accept a service transfer

Accept a Service Transfer for the provided token to receive the services included in the transfer to your account. At this time, only Linodes can be transferred.  When accepted, email confirmations are sent to the accounts that created and accepted the transfer. A transfer can take up to three hours to complete once accepted. Once a transfer is completed, billing for transferred services ends for the sending account and begins for the receiving account.  This operation can only be accessed by the unrestricted users of the account that receives the transfer. Users of the same account that created a transfer cannot accept the transfer.  There are several conditions that must be met in order to accept a transfer request:  1. Only transfers with a `pending` status can be accepted.  1. The account accepting the transfer must have a registered payment method and must not have a past due balance or other account limitations for the services to be transferred.  1. Both the account that created the transfer and the account that is accepting the transfer must not have any active Terms of Service violations.  1. The service must still be owned by the account that created the transfer.  1. Linodes must not:      - be assigned to a NodeBalancer, Firewall, VLAN, or Managed Service.      - have any attached Block Storage Volumes.      - have any shared IP addresses.      - have any assigned /56, /64, or /116 IPv6 ranges.  Any and all of the above conditions must be cured and maintained by the relevant account prior to the transfer's expiration to allow the transfer to be accepted by the receiving account.   <<LB>>  ---   - __CLI__.      ```     linode-cli service-transfers \\   accept 123E4567-E89B-12D3-A456-426614174000     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**token** | **uuid::Uuid** | The UUID of the Service Transfer. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_service_transfer

> models::GetServiceTransfers200ResponseDataInner post_service_transfer(api_version, post_service_transfer_request)
Request a service transfer

Creates a transfer request for the specified services. A request can contain any of the specified service types and any number of each service type. At this time, only Linodes can be transferred.  When created successfully, a confirmation email is sent to the account that created this transfer containing a transfer token and instructions on completing the transfer.  When a transfer is [accepted](https://techdocs.akamai.com/linode-api/reference/post-accept-service-transfer), the requested services are moved to the receiving account. Linode services will not experience interruptions due to the transfer process. Backups for Linodes are transferred as well.  DNS records that are associated with requested services will not be transferred or updated. Please ensure that associated DNS records have been updated or communicated to the recipient prior to the transfer.  A transfer can take up to three hours to complete once accepted. When a transfer is completed, billing for transferred services ends for the sending account and begins for the receiving account.  This operation can only be accessed by the unrestricted users of an account.  There are several conditions that you need to meet to successfully create a transfer request:  1. The account creating the transfer can't have a past due balance or active Terms of Service violation.  1. The service needs to be owned by the account that is creating the transfer.  1. The service can't be assigned to another Service Transfer that is pending or that's been accepted and is incomplete.  1. Linodes can't:      - be assigned to a NodeBalancer, Firewall, VLAN, VPC, or Managed Service.      - have any attached Block Storage Volumes.      - have any shared IP addresses.      - have any assigned /56, /64, or /116 IPv6 ranges.   <<LB>>  ---   - __CLI__.      ```     linode-cli service-transfers \\   create \\   --entities.linodes 111 \\   --entities.linodes 222     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_service_transfer_request** | Option<[**PostServiceTransferRequest**](PostServiceTransferRequest.md)> | The services to include in this transfer request. |  |

### Return type

[**models::GetServiceTransfers200ResponseDataInner**](get_service_transfers_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

