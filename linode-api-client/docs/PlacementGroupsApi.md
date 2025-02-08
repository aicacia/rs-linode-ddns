# \PlacementGroupsApi

All URIs are relative to *https://api.linode.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_placement_group**](PlacementGroupsApi.md#delete_placement_group) | **DELETE** /{apiVersion}/placement/groups/{groupId} | Delete a placement group
[**get_placement_group**](PlacementGroupsApi.md#get_placement_group) | **GET** /{apiVersion}/placement/groups/{groupId} | Get a placement group
[**get_placement_groups**](PlacementGroupsApi.md#get_placement_groups) | **GET** /{apiVersion}/placement/groups | List placement groups
[**post_group_add_linode**](PlacementGroupsApi.md#post_group_add_linode) | **POST** /{apiVersion}/placement/groups/{groupId}/assign | Assign a placement group
[**post_group_unassign**](PlacementGroupsApi.md#post_group_unassign) | **POST** /{apiVersion}/placement/groups/{groupId}/unassign | Unassign a placement group
[**post_placement_group**](PlacementGroupsApi.md#post_placement_group) | **POST** /{apiVersion}/placement/groups | Create placement group
[**put_placement_group**](PlacementGroupsApi.md#put_placement_group) | **PUT** /{apiVersion}/placement/groups/{groupId} | Update a placement group



## delete_placement_group

> serde_json::Value delete_placement_group(api_version, group_id)
Delete a placement group

Deletes a placement group you have permission to `read_write`.  - Deleting a placement group can't be undone. - All compute instances need to be [removed](https://techdocs.akamai.com/linode-api/reference/post-group-unassign) before you can delete a placement group. - If your placement group is non-compliant, you can't delete it. You need to wait for our help to bring it [compliant](https://www.linode.com/docs/products/compute/compute-instances/guides/placement-groups/#non-compliance-precedence).   <<LB>>  ---   - __CLI__.      ```     linode-cli placement group-delete 528     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**group_id** | **i32** | ID of the placement group to look up. Run the [List placement groups](https://techdocs.akamai.com/linode-api/reference/get-placement-groups) operation and store the `id` for the applicable placement group. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_placement_group

> models::GetPlacementGroups200ResponseDataInner get_placement_group(api_version, group_id)
Get a placement group

View a specific placement group by ID.   <<LB>>  ---   - __CLI__.      ```     linode-cli placement group-view 528     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**group_id** | **i32** | ID of the placement group to look up. Run the [List placement groups](https://techdocs.akamai.com/linode-api/reference/get-placement-groups) operation and store the `id` for the applicable placement group. | [required] |

### Return type

[**models::GetPlacementGroups200ResponseDataInner**](get_placement_groups_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_placement_groups

> models::GetPlacementGroups200Response get_placement_groups(api_version, page, page_size)
List placement groups

Returns a paginated list of placement groups you have permission to view.   <<LB>>  ---   - __CLI__.      ```     linode-cli placement groups-list     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     placement:read_only     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**models::GetPlacementGroups200Response**](get_placement_groups_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_group_add_linode

> models::PostPlacementGroup200Response post_group_add_linode(api_version, group_id, post_group_add_linode_request)
Assign a placement group

Add one or more compute instances to your placement group. Check out our [example API workflow](https://www.linode.com/docs/products/compute/compute-instances/guides/placement-groups/#create-a-placement-group) to create a placement group and add compute instances.   <<LB>>  ---   - __CLI__.      ```     linode-cli placement assign-linode 528 \\   --linodes 123 456 \\   --non-compliant true     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**group_id** | **i32** | ID of the placement group to look up. Run the [List placement groups](https://techdocs.akamai.com/linode-api/reference/get-placement-groups) operation and store the `id` for the applicable placement group. | [required] |
**post_group_add_linode_request** | [**PostGroupAddLinodeRequest**](PostGroupAddLinodeRequest.md) | The compute instances you want to add to your placement group. | [required] |

### Return type

[**models::PostPlacementGroup200Response**](post_placement_group_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_group_unassign

> models::PostPlacementGroup200Response post_group_unassign(api_version, group_id, post_group_add_linode_request)
Unassign a placement group

Remove one or more compute instances from your placement group.   <<LB>>  ---   - __CLI__.      ```     linode-cli placement unassign-linode 528 \\   --linode 123 456     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**group_id** | **i32** | ID of the placement group to look up. Run the [List placement groups](https://techdocs.akamai.com/linode-api/reference/get-placement-groups) operation and store the `id` for the applicable placement group. | [required] |
**post_group_add_linode_request** | [**PostGroupAddLinodeRequest**](PostGroupAddLinodeRequest.md) | The compute instances you want to remove from your placement group. | [required] |

### Return type

[**models::PostPlacementGroup200Response**](post_placement_group_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_placement_group

> models::PostPlacementGroup200Response post_placement_group(api_version, post_placement_group_request)
Create placement group

Creates a placement group on your account. Placement groups disperse your compute instances across physical machines (hosts) in one of our compute regions. Depending on your workload requirements, you may need your compute instances to follow specific strategies:  - __Grouped together__. You may want them placed close together to reduce latency between compute instances that are used for an application or workload. - __Spread apart__. You may want to disperse them across several hosts to support high availability, for example when required for failover.  <<LB>>  > 📘 > > - For this request to complete successfully, your user needs to have the `add_placement` grant. > - We offer an [example API workflow](https://www.linode.com/docs/products/compute/compute-instances/guides/placement-groups/#create-a-placement-group) you can follow to create a placement group.   <<LB>>  ---   - __CLI__.      ```     linode-cli placement group-create \\   --label PG_Miami_failover \\   --region us-mia \\   --placement_group_type \"anti-affinity:local\" \\   --placement_group_policy strict     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**post_placement_group_request** | [**PostPlacementGroupRequest**](PostPlacementGroupRequest.md) | The requested initial state of the new placement group. | [required] |

### Return type

[**models::PostPlacementGroup200Response**](post_placement_group_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_placement_group

> models::PostPlacementGroup200Response put_placement_group(api_version, group_id, put_placement_group_request)
Update a placement group

Change the `label` for a specific placement group. This is the only value you can update. However, you can [add](https://techdocs.akamai.com/linode-api/reference/post-group-add-linode) more compute instances or [remove](https://techdocs.akamai.com/linode-api/reference/post-group-unassign) existing ones.   <<LB>>  ---   - __CLI__.      ```     linode-cli placement group-update 528 \\   --label PG_Miami_failover_update     ```      [Learn more...](https://techdocs.akamai.com/cloud-computing/docs/getting-started-with-the-linode-cli)  - __OAuth scopes__.      ```     linodes:read_write     ```      [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | __Enum__ Call either the `v4` URL, or `v4beta` for operations still in Beta. | [required] |
**group_id** | **i32** | ID of the placement group to look up. Run the [List placement groups](https://techdocs.akamai.com/linode-api/reference/get-placement-groups) operation and store the `id` for the applicable placement group. | [required] |
**put_placement_group_request** | [**PutPlacementGroupRequest**](PutPlacementGroupRequest.md) | Include the `label` parameter to update the name of your placement group. | [required] |

### Return type

[**models::PostPlacementGroup200Response**](post_placement_group_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

