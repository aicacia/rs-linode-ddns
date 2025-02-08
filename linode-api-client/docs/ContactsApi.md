# \ContactsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_managed_contact**](ContactsApi.md#delete_managed_contact) | **DELETE** /{apiVersion}/managed/contacts/{contactId} | Delete a managed contact
[**get_managed_contact**](ContactsApi.md#get_managed_contact) | **GET** /{apiVersion}/managed/contacts/{contactId} | Get a managed contact
[**get_managed_contacts**](ContactsApi.md#get_managed_contacts) | **GET** /{apiVersion}/managed/contacts | List managed contacts
[**post_managed_contact**](ContactsApi.md#post_managed_contact) | **POST** /{apiVersion}/managed/contacts | Create a managed contact
[**put_managed_contact**](ContactsApi.md#put_managed_contact) | **PUT** /{apiVersion}/managed/contacts/{contactId} | Update a managed contact



## delete_managed_contact

> serde_json::Value delete_managed_contact(api_version, contact_id)
Delete a managed contact

Deletes a Managed Contact.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed contact-delete 567     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**contact_id** | **i32** | The ID of the contact to access. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_managed_contact

> models::GetManagedContacts200ResponseDataInner get_managed_contact(api_version, contact_id)
Get a managed contact

Returns a single Managed Contact.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed contact-view 567     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**contact_id** | **i32** | The ID of the contact to access. | [required] |

### Return type

[**models::GetManagedContacts200ResponseDataInner**](get_managed_contacts_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_managed_contacts

> models::GetManagedContacts200Response get_managed_contacts(api_version, page, page_size)
List managed contacts

Returns a paginated list of Managed Contacts on your Account.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed contacts-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetManagedContacts200Response**](get_managed_contacts_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_managed_contact

> models::GetManagedContacts200ResponseDataInner post_managed_contact(api_version, post_managed_contact_request)
Create a managed contact

Creates a Managed Contact.  A Managed Contact is someone Linode special forces can contact in the course of attempting to resolve an issue with a Managed Service.  This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed contact-create \\   --name \"John Doe\" \\   --email \"john.doe@example.org\" \\   --phone.primary \"123-456-7890\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_managed_contact_request** | Option<[**PostManagedContactRequest**](PostManagedContactRequest.md)> | Information about the contact to create. |  |

### Return type

[**models::GetManagedContacts200ResponseDataInner**](get_managed_contacts_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_managed_contact

> models::GetManagedContacts200ResponseDataInner put_managed_contact(api_version, contact_id, post_managed_contact_request)
Update a managed contact

Updates information about a Managed Contact. This operation can only be accessed by the unrestricted users of an account.   <<LB>>  ---   - __CLI__.      ```     linode-cli managed contact-update 567 \\   --name \"John Doe\" \\   --email \"john.doe@example.org\" \\   --phone.primary \"123-456-7890\"     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     account:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**contact_id** | **i32** | The ID of the contact to access. | [required] |
**post_managed_contact_request** | [**PostManagedContactRequest**](PostManagedContactRequest.md) | The fields to update. | [required] |

### Return type

[**models::GetManagedContacts200ResponseDataInner**](get_managed_contacts_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

